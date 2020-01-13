mod ast;
mod md;

use crate::ast::Document;
use md::{MdNodeRef, MdRoot, ToMarkdown};

pub trait Documentation {
    fn to_md(&self) -> String;
}

impl Documentation for Document {
    fn to_md(&self) -> String {
        let root = MdNodeRef::new(MdRoot::default());
        self.generate(root.clone());
        format!("{}", root)
    }
}

