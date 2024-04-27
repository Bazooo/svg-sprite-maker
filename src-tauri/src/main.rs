// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod events;
mod config;

use svg_sprite_parser::parser::{get_svg_type_from_file, SvgType};
use tauri::{FileDropEvent, Manager, WindowEvent};
use tauri::api::dialog;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use svg_sprite_parser::symbol::SvgSymbol;
use tauri_specta::Event;
use xml::{EmitterConfig, ParserConfig};
use crate::app::ApplicationState;
use crate::config::{ApplicationConfig, ApplicationConfigSettings, TransparencyGridColor};
use crate::events::{ FilesHoveredEvent, FilesHoverStoppedEvent, SpriteChangedEvent, UnsavedChangesEvent, SettingsChangedEvent, EditorNotSetEvent };

// todo: error handling
// todo: auto-cleanup symbols
// todo: sorting
// todo: undo / redo

fn main() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![
                get_svg_symbol,
                edit_svg_symbol,
                delete_svg_symbols,
                save_new_file,
                save,
                update_symbol_attribute,
                remove_symbol_attribute,
                set_auto_save,
                set_dark_mode,
                get_app_settings,
                set_editor_path,
                set_transparency_grid_colors,
                reset_app_state,
                search_symbols_by_id,
            ])
            .events(tauri_specta::collect_events![
                FilesHoveredEvent,
                FilesHoverStoppedEvent,
                SpriteChangedEvent,
                UnsavedChangesEvent,
                SettingsChangedEvent,
                EditorNotSetEvent
            ]);

        // Only export on non-release builds
        #[cfg(debug_assertions)]
        let specta_builder = specta_builder.path("../src/types/bindings.ts")
            .header("// @ts-nocheck\n// eslint-disable\n");

        specta_builder.into_plugin()
    };

    tauri::Builder::default()
        .plugin(specta_builder)
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

                            let num_symbols = if let Ok(num_symbols) = i32::try_from(num_symbols) {
                                num_symbols
                            } else {
                                -1
                            };

                            FilesHoveredEvent(num_symbols).emit(&target_window).unwrap();
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
                                FilesHoverStoppedEvent().emit(&target_window).unwrap();
                                return;
                            }

                            let state = app_handle.state::<ApplicationState>();
                            state.current_sprite.write().unwrap().append(symbols.as_mut());

                            let current_sprite = state.current_sprite.read().unwrap().clone();

                            FilesHoverStoppedEvent().emit(&target_window).unwrap();
                            SpriteChangedEvent::from(current_sprite).emit(&target_window).unwrap();

                            state.auto_save(target_window.clone());
                        }
                        _ => {
                            FilesHoverStoppedEvent().emit(&target_window).unwrap();
                        },
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(serde::Serialize, specta::Type)]
struct Symbol {
    id: String,
    attributes: HashMap<String, String>,
}

#[tauri::command]
#[specta::specta]
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
#[specta::specta]
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
    SpriteChangedEvent::from(current_sprite).emit(&window).unwrap();

    state.auto_save(window);
}

#[tauri::command]
#[specta::specta]
fn remove_symbol_attribute(symbol_id: &str, key: &str, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    state.current_sprite.write().unwrap().iter_mut()
        .find(|symbol| symbol.id == symbol_id)
        .map(|symbol| {
            symbol.attributes.remove(key);
        });

    let current_sprite = state.current_sprite.read().unwrap().clone();
    SpriteChangedEvent::from(current_sprite).emit(&window).unwrap();

    state.auto_save(window);
}

#[tauri::command(async)]
#[specta::specta]
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
        EditorNotSetEvent().emit(&window).unwrap();
        return;
    };

    // todo: check how to block the command if editor was already opened (VSCode)
    Command::new(editor_path)
        .arg(&temp_file_path)
        .arg("-n")
        .arg("-w")
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
    SpriteChangedEvent::from(current_sprite).emit(&window).unwrap();

    state.auto_save(window);
}

#[tauri::command]
#[specta::specta]
fn delete_svg_symbols(symbol_ids: Vec<&str>, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    state.current_sprite.write().unwrap().retain(|symbol| symbol_ids.iter().all(|&id| id != &symbol.id));
    let current_sprite = state.current_sprite.read().unwrap().clone();
    SpriteChangedEvent::from(current_sprite).emit(&window).unwrap();

    state.auto_save(window);
}

#[tauri::command(async)]
#[specta::specta]
fn save(state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    state.force_save(window);
}

#[tauri::command(async)]
#[specta::specta]
fn save_new_file(state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    *state.unsaved_changes.write().unwrap() = true;
    state.file_path.write().unwrap().take();

    state.force_save(window);
}

#[tauri::command]
#[specta::specta]
fn set_auto_save(enabled: bool, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    let mut app_config = state.config.write().unwrap();

    app_config.update_settings(|builder| {
        builder.auto_save_enabled(enabled)
    });

    SettingsChangedEvent(app_config.settings.clone()).emit(&window).unwrap();
}

#[tauri::command]
#[specta::specta]
fn set_dark_mode(enabled: bool, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    let mut app_config = state.config.write().unwrap();

    app_config.update_settings(|builder| {
        builder.dark_mode(enabled)
    });

    SettingsChangedEvent(app_config.settings.clone()).emit(&window).unwrap();
}

#[tauri::command]
#[specta::specta]
fn get_app_settings(state: tauri::State<'_, ApplicationState>) -> ApplicationConfigSettings {
    state.config.read().unwrap().settings.clone()
}

#[tauri::command]
#[specta::specta]
fn set_editor_path(path: PathBuf, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    let mut app_config = state.config.write().unwrap();

    app_config.update_settings(|builder| {
        builder.editor_path(Some(path))
    });

    SettingsChangedEvent(app_config.settings.clone()).emit(&window).unwrap();
}

#[tauri::command]
#[specta::specta]
fn set_transparency_grid_colors(g1: Option<TransparencyGridColor>, g2: Option<TransparencyGridColor>, state: tauri::State<'_, ApplicationState>, window: tauri::Window) {
    let mut app_config = state.config.write().unwrap();

    app_config.update_settings(|builder| {
        builder.transparency_grid_color_1(g1)
            .transparency_grid_color_2(g2)
    });

    SettingsChangedEvent(app_config.settings.clone()).emit(&window).unwrap();
}

#[tauri::command(async)]
#[specta::specta]
fn reset_app_state(state: tauri::State<'_, ApplicationState>, window: tauri::Window) -> bool {
    let answer = dialog::blocking::confirm(Some(&window), "Reset symbol?", "Are you sure you want to reset the symbol?");

    if !answer {
        return false;
    }

    state.current_sprite.write().unwrap().clear();
    state.file_path.write().unwrap().take();
    *state.unsaved_changes.write().unwrap() = false;

    SpriteChangedEvent::from(Vec::new()).emit(&window).unwrap();
    state.update_window_title(window);

    true
}

#[tauri::command]
#[specta::specta]
fn search_symbols_by_id(query: &str, state: tauri::State<'_, ApplicationState>) -> Vec<String> {
    let sprite = state.current_sprite.read().unwrap();

    if query.is_empty() {
        return sprite.iter().map(|symbol| symbol.id.clone()).collect();
    }

    sprite.iter().filter_map(|symbol| {
        if symbol.id.contains(&query) {
            Some(symbol.id.clone())
        } else {
            None
        }
    }).collect()
}

// todo: filtering symbols by invalid / duplicate / colored
