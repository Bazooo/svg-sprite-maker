use std::path::PathBuf;
use svg::node::element::tag::Type;
use svg::Parser;
use svg::parser::Event;
use crate::symbol::SvgSymbol;
use crate::tag::SvgTag;

#[derive(Debug)]
pub enum SvgType {
    Basic(SvgSymbol),
    Sprite(Vec<SvgSymbol>),
}

pub fn get_svg_type_from_file(file_path: &PathBuf) -> Result<SvgType, ()> {
    let file_ext = file_path.extension();

    let Some(file_ext) = file_ext else {
        return Err(());
    };

    if file_ext.to_string_lossy() != "svg" {
        return Err(());
    }

    let mut svg_content = String::new();
    let Ok(svg_file) = svg::open(file_path, &mut svg_content) else {
        return Err(());
    };

    parse_svg(svg_file)
}

pub(crate) fn parse_svg(svg: Parser) -> Result<SvgType, ()> {
    let events: Vec<_> = svg.into_iter().collect();
    let tags: Vec<_> = events.iter()
        .filter_map(|e| match e {
            Event::Tag(path, tag_type, attributes) => Some(SvgTag {
                path: path.to_string(),
                tag_type: tag_type.clone(),
                attributes: attributes.clone(),
            }),
            _ => None,
        }).collect();

    let Some(root_svg_tag) = tags.iter().find(|t| t.path == "svg") else {
        return Err(());
    };
    let mut symbols = Vec::new();
    let mut current_symbol = SvgSymbol::default();

    for tag in tags.clone() {
        match tag.path.as_str() {
            "symbol" => {
                match tag.tag_type {
                    Type::Start => current_symbol.init(tag.attributes),
                    Type::End => {
                        symbols.push(current_symbol);
                        current_symbol = SvgSymbol::default();
                    }
                    Type::Empty => {},
                }
            },
            "defs" => {},
            "svg" => {},
            _ => {
                if current_symbol.is_initialized() {
                    current_symbol.add_tag(&tag);
                }
            }
        }
    }

    if symbols.len() > 0 {
        return Ok(SvgType::Sprite(symbols));
    }

    current_symbol.init(root_svg_tag.clone().attributes);
    let svg_contents = tags.iter().filter(|t| t.path != "svg").cloned().collect();
    current_symbol.set_tags(svg_contents);

    Ok(SvgType::Basic(current_symbol))
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_svg, SvgType};

    #[test]
    fn test_parse_sprite() {
        let content = r#"
            <svg xmlns="http://www.w3.org/2000/svg">
                <defs>
                    <symbol id="icon-subscript" viewBox="0 0 24 24">
                        <path d="M21.984 18h-1.969v0.984h3v1.031h-4.031v-2.016q0-0.422 0.305-0.703t0.727-0.281h1.969v-1.031h-3v-0.984h3q0.422 0 0.727 0.305t0.305 0.68v1.031q0 0.375-0.305 0.68t-0.727 0.305zM5.859 18h2.672l3.422-5.438h0.094l3.422 5.438h2.672l-4.688-7.266 4.359-6.75h-2.672l-3.094 5.016h-0.094l-3.094-5.016h-2.672l4.313 6.75z"></path>
                    </symbol>
                </defs>
            </svg>
        "#;

        let parser = svg::read(content).unwrap();
        let parsed = parse_svg(parser);

        dbg!(&parsed);
        assert!(matches!(parsed, SvgType::Sprite(_)));
    }

    #[test]
    fn test_parse_basic() {
        let content = r#"
            <svg xmlns="http://www.w3.org/2000/svg">
                <path d="M21.984 18h-1.969v0.984h3v1.031h-4.031v-2.016q0-0.422 0.305-0.703t0.727-0.281h1.969v-1.031h-3v-0.984h3q0.422 0 0.727 0.305t0.305 0.68v1.031q0 0.375-0.305 0.68t-0.727 0.305zM5.859 18h2.672l3.422-5.438h0.094l3.422 5.438h2.672l-4.688-7.266 4.359-6.75h-2.672l-3.094 5.016h-0.094l-3.094-5.016h-2.672l4.313 6.75z"></path>
            </svg>
        "#;

        let parser = svg::read(content).unwrap();
        let parsed = parse_svg(parser);

        dbg!(&parsed);
        assert!(matches!(parsed, SvgType::Basic(_)));
    }
}
