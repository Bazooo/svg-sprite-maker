use svg::node::element::tag::Type;
use svg::node::Attributes;

#[derive(Debug, Clone)]
pub struct SvgTag {
    pub path: String,
    pub tag_type: Type,
    pub attributes: Attributes,
}
