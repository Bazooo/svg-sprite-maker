use svg::node::Attributes;
use crate::tag::SvgTag;

#[derive(Default, Debug)]
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
