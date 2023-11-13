use std::fmt::Display;
use svg::node::element::tag::Type;
use svg::node::Attributes;

#[derive(Debug, Clone)]
pub struct SvgTag {
    pub path: String,
    pub tag_type: Type,
    pub attributes: Attributes,
}

impl Display for SvgTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = vec!["<".to_string()];

        match self.tag_type {
            Type::Start => {
                string.push(self.path.to_string());
                string.push(string_attributes(&self.attributes));
                string.push(">".to_string());
            }
            Type::End => {
                string.push("/".to_string());
                string.push(self.path.to_string());
                string.push(">".to_string());
            },
            Type::Empty => {
                string.push(self.path.to_string());
                string.push(string_attributes(&self.attributes));
                string.push(" />".to_string());
            }
        }

        write!(f, "{}", string.join(""))
    }
}

pub fn string_attributes(attributes: &Attributes) -> String {
    let mut string = vec![];
    for (key, value) in attributes {
        string.push(format!(" {}=\"{}\"", key, value));
    }
    string.join("")
}
