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

impl ApplicationState {
    pub fn update_window_title(&self, window: tauri::Window) {
        let found_path = if let Some(path) = self.file_path.read().unwrap().clone() {
            path.clone().to_string_lossy().to_string()
        } else {
            "Untitled".to_string()
        };

        const APP_NAME: &str = "SVG Sprite Maker";

        let window_title = format!("{} - {}", APP_NAME, found_path);

        if self.unsaved_changes.read().unwrap().clone() {
            window.set_title(&format!("*{}", window_title)).unwrap();
        } else {
            window.set_title(&window_title).unwrap();
        }
    }
}
