// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod events;
mod config;

use svg_sprite_parser::parser::{get_svg_type_from_file, SvgType};
use tauri::{FileDropEvent, Manager, WindowEvent};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use svg_sprite_parser::symbol::SvgSymbol;
use xml::{EmitterConfig, ParserConfig};
use crate::app::ApplicationState;
use crate::config::{ApplicationConfig, ApplicationConfigSettings};
use crate::events::get_sprite;

// todo: dark mode
// todo: auto save
// todo: error handling
// todo: reset / new file
// todo: multi selection delete
// todo: filtering symbols by id / invalid / duplicate / colored
// todo: sorting
// todo: settings

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_svg_symbol,
            edit_svg_symbol,
            delete_svg_symbol,
            set_save_file_path,
            save,
            update_symbol_attribute,
            remove_symbol_attribute,
            set_auto_save,
            get_app_settings,
            set_editor_path,
        ])
        .manage(ApplicationState::default())
        .setup(|app| {
            let binding = app.windows();
            let main_window = binding.get("main").unwrap();
            let target_window = main_window.clone();
            let app_handle = app.handle();

            // Load config
            let state = app.state::<ApplicationState>();
            *state.config.write().unwrap() = ApplicationConfig::load(app_handle.path_resolver().app_config_dir().unwrap().join("config.json"));

            main_window.on_window_event(move |event| {
                if let WindowEvent::FileDrop(file_drop_event) = event {
                    match file_drop_event {
                        FileDropEvent::Hovered(hovered_files) => {
                            let num_symbols = hovered_files.iter()
                                .filter_map(|x| get_svg_type_from_file(x).ok())
                                .fold(0, |count, svg| {
                                    match svg {
                                        SvgType::Basic(_) => count + 1,
                                        SvgType::Sprite(symbols) => count + symbols.len(),
                                    }
                                });

                            target_window.emit(events::FILES_HOVERED, num_symbols).unwrap();
                        }
                        FileDropEvent::Dropped(dropped_files) => {
                            let mut symbols: Vec<_> = dropped_files.iter()
                                .filter_map(|file| get_svg_type_from_file(file).ok())
                                .flat_map(|svg_type| match svg_type {
                                    SvgType::Basic(symbol) => vec![symbol],
                                    SvgType::Sprite(symbols) => symbols,
                                })
                                .collect();

                            if symbols.is_empty() {
                                target_window.emit(events::FILES_HOVER_STOPPED, ()).unwrap();
                                return;
                            }

                            let state = app_handle.state::<ApplicationState>();
                            state.current_sprite.write().unwrap().append(symbols.as_mut());

                            let current_sprite = state.current_sprite.read().unwrap().clone();

                            target_window.emit(events::FILES_HOVER_STOPPED, ()).unwrap();
                            target_window.emit(events::SPRITE_CHANGED, events::SpriteChangedEvent::from(current_sprite)).unwrap();

                            if state.config.read().unwrap().settings.auto_save_enabled.clone() && state.file_path.read().unwrap().is_some() {
                                save(state.clone(), target_window.clone());
                            } else {
                                *state.unsaved_changes.write().unwrap() = true;
                                let path = state.file_path.read().unwrap().clone();

                                target_window.emit(events::UNSAVED_CHANGES, events::UnsavedChangesEvent::from(path)).unwrap();
                                state.update_window_title(target_window.clone());
                            }
                        }
                        _ => {
                            target_window.emit(events::FILES_HOVER_STOPPED, ()).unwrap();
                        },
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(serde::Serialize)]
struct Symbol {
    id: String,
    attributes: HashMap<String, String>,
}

#[tauri::command]
fn get_svg_symbol(symbol_id: &str, state: tauri::State<'_, ApplicationState>) -> Option<Symbol> {
    state.current_sprite.read().unwrap().iter()
        .find(|symbol| symbol.id == symbol_id)
        .map(|symbol| Symbol {
            id: symbol.id.clone(),
            attributes: symbol.attributes.iter().map(|(key, value)| {
                (key.to_string(), value.to_string())
            }).collect(),
        })
}

#[tauri::command]
fn update_symbol_attribute(symbol_id: &str, key: &str, value: &str, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    state.current_sprite.write().unwrap().iter_mut()
        .find(|symbol| symbol.id == symbol_id)
        .map(|symbol| {
            if key == "id" {
                symbol.id = value.to_string();
            }

            symbol.attributes.insert(key.to_string(), value.to_string());
        });

    let current_sprite = state.current_sprite.read().unwrap().clone();
    window.emit(events::SPRITE_CHANGED, events::SpriteChangedEvent::from(current_sprite)).unwrap();
}

#[tauri::command]
fn remove_symbol_attribute(symbol_id: &str, key: &str, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    state.current_sprite.write().unwrap().iter_mut()
        .find(|symbol| symbol.id == symbol_id)
        .map(|symbol| {
            symbol.attributes.remove(key);
        });

    let current_sprite = state.current_sprite.read().unwrap().clone();
    window.emit(events::SPRITE_CHANGED, events::SpriteChangedEvent::from(current_sprite)).unwrap();
}

#[tauri::command(async)]
fn edit_svg_symbol(symbol_id: &str, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    let Some(symbol) = state.current_sprite.read().unwrap().iter().find(|symbol| symbol.id == symbol_id).map(|symbol| symbol.to_string()) else {
        return;
    };

    let mut dest = Vec::new();
    let reader = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(false)
        .create_reader(symbol.as_bytes());
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .normalize_empty_elements(false)
        .autopad_comments(false)
        .create_writer(&mut dest);
    for event in reader {
        if let Some(event) = event.unwrap().as_writer_event() {
            writer.write(event).unwrap();
        }
    }

    let symbol = String::from_utf8(dest).unwrap();

    let mut temp_file = tempfile::Builder::new().suffix(".svg").tempfile().unwrap();

    temp_file.write_all(symbol.as_bytes()).unwrap();

    let temp_file_path = temp_file.into_temp_path();

    let Some(editor_path) = state.config.read().unwrap().settings.editor_path.clone() else {
        window.emit(events::EDITOR_NOT_SET, ()).unwrap();
        return;
    };

    Command::new(editor_path)
        .arg(&temp_file_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap()
        .status;

    let edited_symbol = fs::read_to_string(&temp_file_path).unwrap();

    temp_file_path.close().unwrap();

    let Ok(new_symbol) = SvgSymbol::try_from(edited_symbol) else {
        return;
    };

    state.current_sprite.write().unwrap().iter_mut()
        .find(|symbol| symbol.id == symbol_id)
        .map(|symbol| *symbol = new_symbol);

    let current_sprite = state.current_sprite.read().unwrap().clone();
    window.emit(events::SPRITE_CHANGED, events::SpriteChangedEvent::from(current_sprite)).unwrap();
}

#[tauri::command]
fn delete_svg_symbol(symbol_id: &str, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    state.current_sprite.write().unwrap().retain(|symbol| symbol.id != symbol_id);
    let current_sprite = state.current_sprite.read().unwrap().clone();
    window.emit(events::SPRITE_CHANGED, events::SpriteChangedEvent::from(current_sprite)).unwrap();
}

#[tauri::command]
fn save(state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    let Some(path) = state.file_path.read().unwrap().clone() else {
        window.emit(events::SAVE_FILE_NOT_SET, ()).unwrap();
        return;
    };

    if !state.unsaved_changes.read().unwrap().clone() {
        return;
    }

    let current_sprite = state.current_sprite.read().unwrap().clone();
    fs::write(path, get_sprite(current_sprite)).unwrap();

    *state.unsaved_changes.write().unwrap() = false;
    state.update_window_title(window);
}

#[tauri::command]
fn set_save_file_path(path: PathBuf, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    state.file_path.write().unwrap().replace(path);
    *state.unsaved_changes.write().unwrap() = true;

    save(state, window);
}

#[tauri::command]
fn set_auto_save(enabled: bool, state: tauri::State<'_, ApplicationState>) {
    let mut app_config = state.config.write().unwrap();

    app_config.update_settings(|builder| {
        builder.auto_save_enabled(enabled)
    });
}

#[tauri::command]
fn get_app_settings(state: tauri::State<'_, ApplicationState>) -> ApplicationConfigSettings {
    state.config.read().unwrap().settings.clone()
}

#[tauri::command]
fn set_editor_path(path: PathBuf, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    let mut app_config = state.config.write().unwrap();

    app_config.update_settings(|builder| {
        builder.editor_path(Some(path))
    });

    window.emit(events::SETTINGS_CHANGED, get_app_settings(state.clone())).unwrap();
}
