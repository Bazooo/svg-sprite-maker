// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod events;
mod keybindings;

use svg_sprite_parser::parser::{get_svg_type_from_file, SvgType};
use tauri::{FileDropEvent, Manager, WindowEvent};
use std::collections::HashMap;
use crate::app::ApplicationState;
use crate::keybindings::register_keybindings;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_svg_symbol,
            delete_svg_symbol,
        ])
        .manage(ApplicationState::default())
        .setup(|app| {
            let binding = app.windows();
            let main_window = binding.get("main").unwrap();
            let target_window = main_window.clone();
            let app_handle = app.handle();

            register_keybindings(app_handle.clone());

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

                            target_window.emit_to("main", events::FILES_HOVERED, num_symbols).unwrap();
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
                                target_window.emit_to("main", events::FILES_HOVER_STOPPED, ()).unwrap();
                                return;
                            }

                            let state = app_handle.state::<ApplicationState>();
                            state.current_sprite.write().unwrap().append(symbols.as_mut());

                            let current_sprite = state.current_sprite.read().unwrap().clone();

                            target_window.emit_to("main", events::FILES_HOVER_STOPPED, ()).unwrap();
                            target_window.emit_to("main", events::SPRITE_CHANGED, events::SpriteChangedEvent::from(current_sprite)).unwrap();
                        }
                        _ => {
                            target_window.emit_to("main", events::FILES_HOVER_STOPPED, ()).unwrap();
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
fn delete_svg_symbol(symbol_id: &str, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    state.current_sprite.write().unwrap().retain(|symbol| symbol.id != symbol_id);
    let current_sprite = state.current_sprite.read().unwrap().clone();
    window.emit_to("main", events::SPRITE_CHANGED, events::SpriteChangedEvent::from(current_sprite)).unwrap();
}
