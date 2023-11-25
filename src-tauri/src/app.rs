use std::path::PathBuf;
use std::sync::RwLock;
use svg_sprite_parser::symbol::SvgSymbol;

#[derive(Default)]
pub struct ApplicationState {
    pub current_sprite: RwLock<Vec<SvgSymbol>>,
    pub file_path: RwLock<Option<PathBuf>>,
    pub auto_save_enabled: RwLock<bool>,
    pub unsaved_changes: RwLock<bool>,
}
