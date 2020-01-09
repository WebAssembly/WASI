use std::{
    cell::{Ref, RefCell, RefMut},
    fmt,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub enum MdElement {
    Section(RefCell<MdSection>),
    TypeListing(RefCell<MdTypeListing>),
    InterfaceFunc(RefCell<MdInterfaceFunc>),
}

impl MdElement {
    pub fn as_section(&self) -> Ref<MdSection> {
        match self {
            Self::Section(t) => t.borrow(),
            _ => panic!("not a MdSection"),
        }
    }

    pub fn as_section_mut(&self) -> RefMut<MdSection> {
        match self {
            Self::Section(t) => t.borrow_mut(),
            _ => panic!("not a MdSection"),
        }
    }

    pub fn as_type_listing(&self) -> Ref<MdTypeListing> {
        match self {
            Self::TypeListing(t) => t.borrow(),
            _ => panic!("not a MdTypeListing"),
        }
    }

    pub fn as_type_listing_mut(&self) -> RefMut<MdTypeListing> {
        match self {
            Self::TypeListing(t) => t.borrow_mut(),
            _ => panic!("not a MdTypeListing"),
        }
    }

    pub fn as_interface_func(&self) -> Ref<MdInterfaceFunc> {
        match self {
            Self::InterfaceFunc(t) => t.borrow(),
            _ => panic!("not a MdInterfaceFunc"),
        }
    }

    pub fn as_interface_func_mut(&self) -> RefMut<MdInterfaceFunc> {
        match self {
            Self::InterfaceFunc(t) => t.borrow_mut(),
            _ => panic!("not a MdInterfaceFunc"),
        }
    }

    pub fn parent(&self) -> Option<Rc<MdElement>> {
        match self {
            Self::Section(t) => t.borrow().parent.as_ref().and_then(|x| x.upgrade()),
            Self::TypeListing(t) => t.borrow().parent.as_ref().and_then(|x| x.upgrade()),
            Self::InterfaceFunc(t) => t.borrow().parent.as_ref().and_then(|x| x.upgrade()),
        }
    }
}

impl fmt::Display for MdElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Section(t) => t.borrow().fmt(f),
            Self::TypeListing(t) => t.borrow().fmt(f),
            Self::InterfaceFunc(t) => t.borrow().fmt(f),
        }
    }
}

fn walk_parents(start: &Weak<MdElement>, cb: &mut impl FnMut(Rc<MdElement>)) {
    let mut parent = if let Some(parent) = start.upgrade() {
        cb(parent.clone());
        parent
    } else {
        return;
    };

    while let Some(p) = parent.parent() {
        cb(p.clone());
        parent = p;
    }
}

#[derive(Debug)]
pub struct MdSection {
    pub id: String,
    pub title: String,
    pub description: Vec<String>,
    pub elements: Vec<Rc<MdElement>>,
    pub parent: Option<Weak<MdElement>>,
}

impl MdSection {
    pub fn new(id: &str, title: &str, parent: Option<Weak<MdElement>>) -> MdElement {
        MdElement::Section(RefCell::new(Self {
            id: id.to_owned(),
            title: title.to_owned(),
            description: vec![],
            elements: vec![],
            parent,
        }))
    }
}

impl fmt::Display for MdSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut heading = "#".to_owned();
        if let Some(parent) = self.parent.as_ref() {
            walk_parents(parent, &mut |_| {
                heading += "#";
            });
        }
        f.write_fmt(format_args!(
            "{heading} <a href=\"#{id}\" name=\"{id}\"></a> {title}\n",
            heading = heading,
            id = self.id,
            title = self.title
        ))?;
        for para in &self.description {
            f.write_fmt(format_args!("{}\n", para))?;
        }
        for el in &self.elements {
            f.write_fmt(format_args!("{}\n", el))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct MdTypeListing {
    pub id: String,
    pub r#type: MdType,
    pub description: Vec<String>,
    pub elements: Vec<MdBullet>,
    pub parent: Option<Weak<MdElement>>,
}

#[derive(Debug)]
pub enum MdType {
    Enum { repr: String },
    Int { repr: String },
    Flags { repr: String },
    Struct,
    Union,
    Handle,
    Array { r#type: String },
    Pointer { to: String },
    ConstPointer { to: String },
    Builtin { r#type: String },
}

#[derive(Debug)]
pub struct MdBullet {
    pub id: String,
    pub description: String,
}

impl MdTypeListing {
    pub fn new(id: &str, r#type: MdType, parent: Option<Weak<MdElement>>) -> MdElement {
        MdElement::TypeListing(RefCell::new(Self {
            id: id.to_owned(),
            r#type,
            description: vec![],
            elements: vec![],
            parent,
        }))
    }
}

impl fmt::Display for MdTypeListing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut heading = "#".to_owned();
        if let Some(parent) = self.parent.as_ref() {
            walk_parents(parent, &mut |_| {
                heading += "#";
            });
        }
        // ### <a href="#errno" name="errno"></a> `errno`
        f.write_fmt(format_args!(
            "{heading} <a href=\"#{id}\" name=\"{id}\"></a> `{id}`\n",
            heading = heading,
            id = self.id
        ))?;
        // Error codes returned by function...
        for para in &self.description {
            f.write_fmt(format_args!("{}\n", para))?;
        }
        // Enum represented by `u16`
        // Variants:
        let type_specific = match &self.r#type {
            MdType::Enum { repr } => format!("Enum represented by `{}`\n\n**Variants:**\n", repr),
            MdType::Int { repr } => format!("Int represented by `{}`\n\n**Const:**\n", repr),
            MdType::Flags { repr } => format!("Flags represented by `{}`\n\n**Flags:**\n", repr),
            MdType::Struct => "\n**Struct members:**\n".to_owned(),
            MdType::Union => "\n**Union variants:**\n".to_owned(),
            MdType::Handle => "\n**Supertypes:**\n".to_owned(),
            MdType::Array { r#type } => format!("Array of `{}`", r#type),
            MdType::Pointer { to } => format!("Pointer to `{}`", to),
            MdType::ConstPointer { to } => format!("Const pointer to `{}`", to),
            MdType::Builtin { r#type } => format!("Builtin type `{}`", r#type),
        };
        f.write_str(&type_specific)?;
        // - <a href="#errno::success", name="errno::success"></a> `success`
        //   No error occurred. System call completed successfully.
        for el in &self.elements {
            f.write_fmt(format_args!(
                "- <a href=\"#{this_id}.{id}\" name=\"{this_id}.{id}\"></a> `{id}`\n\n",
                this_id = self.id,
                id = el.id
            ))?;
            f.write_fmt(format_args!("\t{}\n", &el.description))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct MdInterfaceFunc {
    pub id: String,
    pub description: Vec<String>,
    pub parameters: Vec<MdBullet>,
    pub results: Vec<MdBullet>,
    pub parent: Option<Weak<MdElement>>,
}

impl MdInterfaceFunc {
    pub fn new(id: &str, parent: Option<Weak<MdElement>>) -> MdElement {
        MdElement::InterfaceFunc(RefCell::new(Self {
            id: id.to_owned(),
            description: vec![],
            parameters: vec![],
            results: vec![],
            parent,
        }))
    }
}

impl fmt::Display for MdInterfaceFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut heading = "#".to_owned();
        if let Some(parent) = self.parent.as_ref() {
            walk_parents(parent, &mut |_| {
                heading += "#";
            });
        }
        // ### <a href="#args_get" name="args_get"></a> `args_get`
        f.write_fmt(format_args!(
            "{heading} <a href=\"#{id}\" name=\"{id}\"></a> `{id}`\n\n",
            heading = heading,
            id = self.id
        ))?;
        // Read command-line argument data...
        for desc in &self.description {
            f.write_fmt(format_args!("{}\n", desc))?;
        }
        // Parameters:
        // * `argv`
        //   `argv` has type...
        f.write_str("\n**Parameters:**\n\n")?;
        for param in &self.parameters {
            f.write_fmt(format_args!(
                "- <a href=\"{id}.{param_id}\" name=\"{id}.{param_id}\"></a> `{param_id}`\n\n",
                id = self.id,
                param_id = param.id
            ))?;
            f.write_fmt(format_args!("\t{}\n", &param.description))?;
        }
        // Results:
        // * `error`
        //   `error` has type `errno`
        f.write_str("\n**Results:**\n\n")?;
        for result in &self.results {
            f.write_fmt(format_args!(
                "- <a href=\"{id}.{res_id}\" name=\"{id}.{res_id}\"></a> `{res_id}`\n\n",
                id = self.id,
                res_id = result.id
            ))?;
            f.write_fmt(format_args!("\t{}\n", &result.description))?;
        }
        Ok(())
    }
}
