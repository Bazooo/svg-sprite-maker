use std::collections::HashMap;
use std::fmt::Display;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::parser::{get_svg_symbols, get_svg_tags, parse_svg, SvgType};
use crate::tag::{string_attributes, SvgTag};

#[derive(Default, Debug, Clone)]
pub struct SvgSymbol {
    pub id: String,
    pub contents: Vec<SvgTag>,
    pub attributes: HashMap<String, String>,
    is_initialized: bool,
}

impl SvgSymbol {
    pub fn init(&mut self, attributes: HashMap<String, String>) {
        let mut attributes = attributes.clone();

        let id = match attributes.get("id") {
            Some(id_attribute) => id_attribute.to_string(),
            None => {
                let gen_id = format!("svg-sprite-{}", generate_id());
                attributes.insert("id".to_string(), gen_id.to_string());

                gen_id
            },
        };

        self.id = id.to_string();
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

impl TryFrom<String> for SvgSymbol {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let svg = svg::read(&value).map_err(|_| ())?;

        let tags = get_svg_tags(svg);
        let symbols = get_svg_symbols(tags);

        if symbols.len() != 1 {
            return Err(());
        }

        Ok(symbols.first().unwrap().clone())
    }
}

pub fn generate_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}
