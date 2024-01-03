use std::path::PathBuf;
use svg_sprite_parser::symbol::SvgSymbol;
use crate::config::ApplicationConfigSettings;

#[derive(serde::Serialize, Clone, specta::Type, tauri_specta::Event)]
pub struct FilesHoveredEvent(pub i32);

#[derive(serde::Serialize, Clone, specta::Type, tauri_specta::Event)]
pub struct FilesHoverStoppedEvent();

#[derive(serde::Serialize, Clone, specta::Type, tauri_specta::Event)]
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

pub fn get_sprite(symbols: Vec<SvgSymbol>) -> String {
    let mut string = vec!["<svg xmlns=\"http://www.w3.org/2000/svg\"><defs>".to_string()];

    for symbol in symbols {
        string.push(symbol.to_string());
    }

    string.push("</defs></svg>".to_string());
    string.join("")
}

#[derive(serde::Serialize, Clone, specta::Type, tauri_specta::Event)]
pub struct UnsavedChangesEvent {
    pub path: Option<PathBuf>,
}

impl From<Option<PathBuf>> for UnsavedChangesEvent {
    fn from(path: Option<PathBuf>) -> Self {
        Self { path }
    }
}

#[derive(serde::Serialize, Clone, specta::Type, tauri_specta::Event)]
pub struct SettingsChangedEvent(pub ApplicationConfigSettings);

#[derive(serde::Serialize, Clone, specta::Type, tauri_specta::Event)]
pub struct EditorNotSetEvent();
