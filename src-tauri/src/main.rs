// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod events;

use svg_sprite_parser::parser::{get_svg_type_from_file, SvgType};
use tauri::{FileDropEvent, Manager, WindowEvent};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::app::ApplicationState;
use crate::events::get_sprite;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_svg_symbol,
            delete_svg_symbol,
            set_save_file_path,
            save,
            update_symbol_attribute,
            remove_symbol_attribute,
            set_auto_save,
        ])
        .manage(ApplicationState::default())
        .setup(|app| {
            let binding = app.windows();
            let main_window = binding.get("main").unwrap();
            let target_window = main_window.clone();
            let app_handle = app.handle();

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

                            if state.auto_save_enabled.read().unwrap().clone() && state.file_path.read().unwrap().is_some() {
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
    *state.auto_save_enabled.write().unwrap() = enabled;
}
