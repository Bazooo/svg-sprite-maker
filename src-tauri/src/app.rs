use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;
use svg_sprite_parser::symbol::SvgSymbol;
use tauri::api::dialog;
use crate::config::ApplicationConfig;
use crate::events;
use crate::events::get_sprite;

#[derive(Default)]
pub struct ApplicationState {
    pub current_sprite: RwLock<Vec<SvgSymbol>>,
    pub file_path: RwLock<Option<PathBuf>>,
    pub unsaved_changes: RwLock<bool>,
    pub config: RwLock<ApplicationConfig>,
}

impl ApplicationState {
    pub fn update_window_title(&self, window: tauri::Window) {
        let fp = self.file_path.read().unwrap().clone();

        let found_path = if let Some(path) = fp.clone() {
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

    pub fn auto_save(&self, window: tauri::Window) {
        *self.unsaved_changes.write().unwrap() = true;

        if self.config.read().unwrap().settings.auto_save_enabled.clone() && self.file_path.read().unwrap().is_some() {
            self.force_save(window);
        } else {
            let path = self.file_path.read().unwrap().clone();

            window.emit(events::UNSAVED_CHANGES, events::UnsavedChangesEvent::from(path)).unwrap();
            self.update_window_title(window);
        }
    }

    pub fn force_save(&self, window: tauri::Window) {
        let Some(file_path) = self.get_save_file_path() else {
            return;
        };

        self.save_to_file(file_path);
        self.update_window_title(window);
    }

    fn get_save_file_path(&self) -> Option<PathBuf> {
        let mut file_path_state = self.file_path.write().unwrap();

        if let Some(file_path) = file_path_state.clone() {
            Some(file_path)
        } else {
            let possible_file_path = dialog::blocking::FileDialogBuilder::new()
                .add_filter("SVG Sprite", &["svg"])
                .save_file();

            let Some(file_path) = possible_file_path else {
                return None;
            };

            file_path_state.replace(file_path.clone());

            Some(file_path)
        }
    }

    fn save_to_file(&self, file_path: PathBuf) {
        let mut unsaved_changes_state = self.unsaved_changes.write().unwrap();

        if !unsaved_changes_state.clone() {
            return;
        }

        let current_sprite = self.current_sprite.read().unwrap().clone();
        fs::write(file_path, get_sprite(current_sprite)).unwrap();

        *unsaved_changes_state = false;
    }
}
