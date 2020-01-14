use std::{
    any::Any,
    cell::{self, RefCell},
    fmt,
    rc::{Rc, Weak},
};

pub(super) trait ToMarkdown {
    fn generate(&self, node: MdNodeRef);
}

pub(super) trait MdElement: fmt::Display + fmt::Debug + 'static {
    fn id(&self) -> Option<&str>;
    fn docs(&self) -> Option<&str>;
    fn set_docs(&mut self, docs: &str);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
pub(super) struct MdNode {
    content: Box<dyn MdElement>,
    parent: Option<Weak<RefCell<MdNode>>>,
    children: Vec<MdNodeRef>,
}

fn walk_parents(parent: Option<&Weak<RefCell<MdNode>>>, cb: &mut impl FnMut(MdNodeRef)) {
    if let Some(parent) = parent.and_then(|x| x.upgrade()) {
        cb(parent.clone().into());
        walk_parents(parent.borrow().parent.as_ref(), cb)
    }
}

impl MdNode {
    fn new<T: MdElement + 'static>(item: T) -> Self {
        Self {
            content: Box::new(item),
            parent: None,
            children: vec![],
        }
    }

    pub fn parents(&self) -> Vec<MdNodeRef> {
        let mut parents = Vec::new();
        walk_parents(self.parent.as_ref(), &mut |parent| parents.push(parent));
        parents
    }

    pub fn children(&self) -> Vec<MdNodeRef> {
        let mut children = self.children.clone();
        for child in &self.children {
            children.append(&mut child.borrow().children());
        }
        children
    }
}

impl fmt::Display for MdNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.content.fmt(f)?;

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

    pub fn borrow(&self) -> cell::Ref<MdNode> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> cell::RefMut<MdNode> {
        self.0.borrow_mut()
    }

    pub fn any_ref(&self) -> cell::Ref<Box<dyn MdElement>> {
        cell::Ref::map(self.borrow(), |b| &b.content)
    }

    pub fn any_ref_mut(&self) -> cell::RefMut<Box<dyn MdElement>> {
        cell::RefMut::map(self.borrow_mut(), |b| &mut b.content)
    }

    pub fn content_mut<T: MdElement + 'static>(&self) -> cell::RefMut<T> {
        cell::RefMut::map(self.borrow_mut(), |b| {
            let r = b.content.as_any_mut();
            r.downcast_mut::<T>().expect("reference is not T type")
        })
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

    fn docs(&self) -> Option<&str> {
        None
    }

    fn set_docs(&mut self, _: &str) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for MdRoot {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct MdSection {
    pub header: String,
    pub id: Option<String>,
    pub title: String,
}

impl MdSection {
    pub fn new<S: AsRef<str>>(header: S, title: S) -> Self {
        Self {
            header: header.as_ref().to_owned(),
            id: None,
            title: title.as_ref().to_owned(),
        }
    }
}

impl MdElement for MdSection {
    fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|s| s.as_str())
    }

    fn docs(&self) -> Option<&str> {
        None
    }

    fn set_docs(&mut self, _: &str) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn gen_link<S: AsRef<str>>(id: S) -> String {
    format!("<a href=\"#{id}\" name=\"{id}\"></a>", id = id.as_ref())
}

impl fmt::Display for MdSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{} ", self.header))?;

        if let Some(id) = &self.id {
            f.write_fmt(format_args!("{} ", gen_link(id)))?;
        }

        writeln!(f, "{}", self.title)
    }
}

#[derive(Debug)]
pub(super) struct MdNamedType {
    pub header: String,
    pub id: String,
    pub name: String,
    pub docs: String,
    pub r#type: Option<MdType>,
}

impl MdNamedType {
    pub fn new<S: AsRef<str>>(header: S, id: S, name: S, docs: S) -> Self {
        Self {
            header: header.as_ref().to_owned(),
            id: id.as_ref().to_owned(),
            name: name.as_ref().to_owned(),
            docs: docs.as_ref().to_owned(),
            r#type: None,
        }
    }
}

// TODO `MdType` should probably store `TypeRef` and recursively
// unwind itself into final `String` representation rather than
// being outright flattened.
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
        Some(&self.id)
    }

    fn docs(&self) -> Option<&str> {
        Some(&self.docs)
    }

    fn set_docs(&mut self, docs: &str) {
        self.docs = docs.to_owned();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for MdNamedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "{header} {link} `{name}`",
            header = self.header,
            link = gen_link(&self.id),
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
    pub header: String,
    pub id: String,
    pub name: String,
    pub inputs: Vec<(String, String)>,
    pub outputs: Vec<String>,
    pub docs: String,
}

impl MdFunc {
    pub fn new<S: AsRef<str>>(header: S, id: S, name: S, docs: S) -> Self {
        Self {
            header: header.as_ref().to_owned(),
            id: id.as_ref().to_owned(),
            name: name.as_ref().to_owned(),
            inputs: vec![],
            outputs: vec![],
            docs: docs.as_ref().to_owned(),
        }
    }
}

impl MdElement for MdFunc {
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn docs(&self) -> Option<&str> {
        Some(&self.docs)
    }

    fn set_docs(&mut self, docs: &str) {
        self.docs = docs.to_owned();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for MdFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
            "{header} {link} Fn {name}({inputs}){outputs}",
            header = self.header,
            link = gen_link(&self.id),
            name = self.name,
            inputs = inputs,
            outputs = outputs,
        ))?;

        writeln!(f, "\n{}", self.docs)
    }
}
