use std::fmt::Display;
use svg::node::Attributes;
use crate::tag::{string_attributes, SvgTag};

#[derive(Default, Debug, Clone)]
pub struct SvgSymbol {
    pub contents: Vec<SvgTag>,
    pub attributes: Attributes,
    is_initialized: bool,
}

impl SvgSymbol {
    pub fn init(&mut self, attributes: Attributes) {
        self.attributes = attributes;
        self.is_initialized = true;
    }

    pub fn add_tag(&mut self, tag: &SvgTag) {
        self.contents.push(tag.clone());
    }

    pub fn set_tags(&mut self, tags: Vec<SvgTag>) {
        self.contents = tags;
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Display for SvgSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = vec!["<symbol".to_string()];
        string.push(string_attributes(&self.attributes));
        string.push(">".to_string());

        for tag in self.contents.clone() {
            let tag = tag.to_string();
            string.push(tag);
        }

        string.push("</symbol>".to_string());
        write!(f, "{}", string.join(""))
    }
}
