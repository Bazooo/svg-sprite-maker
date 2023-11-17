use svg_sprite_parser::symbol::SvgSymbol;

pub const FILES_HOVERED: &str = "files-hovered";
pub const FILES_HOVER_STOPPED: &str = "files-hover-stopped";
pub const SPRITE_CHANGED: &str = "sprite-changed";

#[derive(serde::Serialize, Clone)]
pub struct SpriteChangedEvent {
    pub ids: Vec<String>,
    pub sprite: String,
}

impl From<Vec<SvgSymbol>> for SpriteChangedEvent {
    fn from(symbols: Vec<SvgSymbol>) -> Self {
        Self {
            ids: symbols
                .iter()
                .map(|symbol| symbol.id.to_string())
                .collect(),
            sprite: get_sprite(symbols),
        }
    }
}

fn get_sprite(symbols: Vec<SvgSymbol>) -> String {
    let mut string = vec!["<svg xmlns=\"http://www.w3.org/2000/svg\"><defs>".to_string()];

    for symbol in symbols {
        string.push(symbol.to_string());
    }

    string.push("</defs></svg>".to_string());
    string.join("")
}
