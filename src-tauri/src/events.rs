use std::path::PathBuf;
use svg_sprite_parser::symbol::SvgSymbol;

pub const FILES_HOVERED: &str = "files-hovered";
pub const FILES_HOVER_STOPPED: &str = "files-hover-stopped";
pub const SPRITE_CHANGED: &str = "sprite-changed";
pub const UNSAVED_CHANGES: &str = "unsaved-changes";
pub const SETTINGS_CHANGED: &str = "settings-changed";
pub const EDITOR_NOT_SET: &str = "editor-not-set";

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

#[derive(serde::Serialize, Clone)]
pub struct UnsavedChangesEvent {
    pub path: Option<PathBuf>,
}

impl From<Option<PathBuf>> for UnsavedChangesEvent {
    fn from(path: Option<PathBuf>) -> Self {
        Self { path }
    }
}

pub fn get_sprite(symbols: Vec<SvgSymbol>) -> String {
    let mut string = vec!["<svg xmlns=\"http://www.w3.org/2000/svg\"><defs>".to_string()];

    for symbol in symbols {
        string.push(symbol.to_string());
    }

    string.push("</defs></svg>".to_string());
    string.join("")
}
