use std::{
    any::Any,
    cell::{self, RefCell},
    fmt,
    rc::{Rc, Weak},
};

pub(super) trait ToMarkdown {
    fn generate(&self, node: MdNodeRef);
}

pub(super) trait MdElement: fmt::Debug + 'static {
    fn id(&self) -> Option<&str>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn fmt(&self, f: &mut fmt::Formatter, parents: Vec<MdNodeRef>) -> fmt::Result;
}

#[derive(Debug)]
struct MdNode {
    content: Box<dyn MdElement>,
    parent: Option<Weak<RefCell<MdNode>>>,
    children: Vec<MdNodeRef>,
}

impl MdNode {
    fn new<T: MdElement + 'static>(item: T) -> Self {
        Self {
            content: Box::new(item),
            parent: None,
            children: vec![],
        }
    }
}

fn walk_parents(parent: Option<&Weak<RefCell<MdNode>>>, cb: &mut impl FnMut(MdNodeRef)) {
    if let Some(parent) = parent.and_then(|x| x.upgrade()) {
        cb(parent.clone().into());
        walk_parents(parent.borrow().parent.as_ref(), cb)
    }
}

impl fmt::Display for MdNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parents = Vec::new();
        walk_parents(self.parent.as_ref(), &mut |parent| {
            parents.push(parent);
        });

        MdElement::fmt(&*self.content, f, parents)?;

        for child in &self.children {
            child.fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct MdNodeRef(Rc<RefCell<MdNode>>);

impl MdNodeRef {
    pub fn new<T: MdElement + 'static>(item: T) -> Self {
        Self(Rc::new(RefCell::new(MdNode::new(item))))
    }

    pub fn new_child<T: MdElement + 'static>(&self, item: T) -> Self {
        let mut child_node = MdNode::new(item);
        child_node.parent = Some(Rc::downgrade(&self.0));
        let child_ref = Self(Rc::new(RefCell::new(child_node)));
        self.borrow_mut().children.push(child_ref.clone());
        child_ref
    }

    fn borrow(&self) -> cell::Ref<MdNode> {
        self.0.borrow()
    }

    fn borrow_mut(&self) -> cell::RefMut<MdNode> {
        self.0.borrow_mut()
    }

    pub fn content_ref<T: MdElement + 'static>(&self) -> cell::Ref<T> {
        cell::Ref::map(self.borrow(), |b| {
            let r = b.content.as_any();
            r.downcast_ref::<T>().expect("reference is not T type")
        })
    }

    pub fn content_mut<T: MdElement + 'static>(&self) -> cell::RefMut<T> {
        cell::RefMut::map(self.borrow_mut(), |b| {
            let r = b.content.as_any_mut();
            r.downcast_mut::<T>().expect("reference is not T type")
        })
    }

    pub fn get_content<T: MdElement + 'static>(&self) -> Option<cell::Ref<T>> {
        if self.borrow().content.as_any().is::<T>() {
            Some(self.content_ref::<T>())
        } else {
            None
        }
    }

    #[allow(unused)]
    pub fn get_content_mut<T: MdElement + 'static>(&self) -> Option<cell::RefMut<T>> {
        if self.borrow_mut().content.as_any().is::<T>() {
            Some(self.content_mut::<T>())
        } else {
            None
        }
    }
}

impl Clone for MdNodeRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl From<Rc<RefCell<MdNode>>> for MdNodeRef {
    fn from(node: Rc<RefCell<MdNode>>) -> Self {
        Self(node)
    }
}

impl fmt::Display for MdNodeRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.borrow().fmt(f)
    }
}

#[derive(Debug, Default)]
pub(super) struct MdRoot;

impl MdElement for MdRoot {
    fn id(&self) -> Option<&str> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn fmt(&self, _f: &mut fmt::Formatter, _parents: Vec<MdNodeRef>) -> fmt::Result {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(super) struct MdSection {
    pub id: Option<String>,
    pub title: String,
}

impl MdSection {
    pub fn new<S: AsRef<str>>(title: S) -> Self {
        Self {
            id: None,
            title: title.as_ref().to_owned(),
        }
    }
}

impl MdElement for MdSection {
    fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|s| s.as_str())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn fmt(&self, f: &mut fmt::Formatter, parents: Vec<MdNodeRef>) -> fmt::Result {
        let header = "#".repeat(parents.len());
        f.write_fmt(format_args!("{} ", header))?;

        if let Some(id) = &self.id {
            f.write_fmt(format_args!(
                "<a href=\"#{id}\" name=\"{id}\"></a> ",
                id = id
            ))?;
        }

        writeln!(f, "{}", self.title)
    }
}

#[derive(Debug)]
pub(super) struct MdNamedType {
    pub name: String,
    pub docs: String,
    pub r#type: Option<MdType>,
}

impl MdNamedType {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, docs: S2) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            docs: docs.as_ref().to_owned(),
            r#type: None,
        }
    }
}

#[derive(Debug)]
pub(super) enum MdType {
    Enum { repr: String },
    Int { repr: String },
    Flags { repr: String },
    Struct,
    Union,
    Array { r#type: String },
    Pointer { r#type: String },
    ConstPointer { r#type: String },
    Builtin { repr: String },
    Handle,
    Alias { r#type: String },
}

impl fmt::Display for MdType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Enum { repr } => f.write_fmt(format_args!(": Enum(`{}`)", repr))?,
            Self::Int { repr } => f.write_fmt(format_args!(": Int(`{}`)", repr))?,
            Self::Flags { repr } => f.write_fmt(format_args!(": Flags(`{}`)", repr))?,
            Self::Struct => f.write_fmt(format_args!(": Struct"))?,
            Self::Union => f.write_fmt(format_args!(": Union"))?,
            Self::Array { r#type } => f.write_fmt(format_args!(": `Array<{}>`", r#type))?,
            Self::Pointer { r#type } => f.write_fmt(format_args!(": `Pointer<{}>`", r#type))?,
            Self::ConstPointer { r#type } => {
                f.write_fmt(format_args!(": `ConstPointer<{}>`", r#type))?
            }
            Self::Builtin { repr } => f.write_fmt(format_args!(": `{}`", repr))?,
            Self::Handle => {}
            Self::Alias { r#type } => {
                f.write_fmt(format_args!(": [`{tt}`](#{tt})", tt = r#type))?
            }
        };

        Ok(())
    }
}

impl MdElement for MdNamedType {
    fn id(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn fmt(&self, f: &mut fmt::Formatter, parents: Vec<MdNodeRef>) -> fmt::Result {
        // Firstly, check if our parent is an MdSection or another MdNamedType in which
        // case, we're already nesting and should be represented as bullets
        let (header, link) = if let Some(p) = parents.first() {
            if let Some(_sec) = p.get_content::<MdSection>() {
                ("#".repeat(parents.len()), self.name.clone())
            } else if let Some(tt) = p.get_content::<MdNamedType>() {
                ("-".to_owned(), format!("{}.{}", tt.name, self.name))
            } else if let Some(f) = p.get_content::<MdFunc>() {
                ("-".to_owned(), format!("{}.{}", f.name, self.name))
            } else {
                ("#".to_owned(), self.name.clone())
            }
        } else {
            ("#".to_owned(), self.name.clone())
        };

        f.write_fmt(format_args!(
            "{header} <a href=\"#{link}\" name=\"{link}\"></a> `{name}`",
            header = header,
            link = link,
            name = self.name,
        ))?;

        if let Some(tt) = &self.r#type {
            f.write_fmt(format_args!("{}", tt))?;
        }

        writeln!(f, "\n{}", self.docs)
    }
}

#[derive(Debug)]
pub(super) struct MdFunc {
    pub name: String,
    pub inputs: Vec<(String, String)>,
    pub outputs: Vec<String>,
    pub docs: String,
}

impl MdFunc {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, docs: S2) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            inputs: vec![],
            outputs: vec![],
            docs: docs.as_ref().to_owned(),
        }
    }
}

impl MdElement for MdFunc {
    fn id(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn fmt(&self, f: &mut fmt::Formatter, parents: Vec<MdNodeRef>) -> fmt::Result {
        // Expand header
        let header = "#".repeat(parents.len());
        // Expand inputs
        let inputs = self
            .inputs
            .iter()
            .map(|(name, r#type)| format!("{}: `{}`", name, r#type))
            .collect::<Vec<_>>()
            .join(", ");
        // Expand outputs
        let outputs: Vec<_> = self
            .outputs
            .iter()
            .map(|r#type| format!("`{}`", r#type))
            .collect();
        let outputs = match outputs.len() {
            0 => "".to_owned(),
            1 => format!(" -> {}", outputs[0]),
            _ => format!(" -> ({})", outputs.join(", ")),
        };
        // Format
        writeln!(f, "\n---\n")?;

        f.write_fmt(format_args!(
            "{header} <a href=\"#{name}\" name=\"{name}\"></a> Fn {name}({inputs}){outputs}",
            header = header,
            name = self.name,
            inputs = inputs,
            outputs = outputs,
        ))?;

        writeln!(f, "\n{}", self.docs)
    }
}
