// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use svg_sprite_parser::parser::{get_svg_type_from_file, SvgType};
use svg_sprite_parser::symbol::SvgSymbol;
use tauri::{FileDropEvent, Manager, WindowEvent};
use crate::app::ApplicationState;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_current_sprite])
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

                            target_window.emit_to("main", "files-hovered", num_symbols).unwrap();
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
                                target_window.emit_to("main", "files-hover-stopped", ()).unwrap();
                                return;
                            }

                            let state = app_handle.state::<ApplicationState>();
                            state.current_sprite.write().unwrap().append(symbols.as_mut());

                            let current_sprite = state.current_sprite.read().unwrap().clone();

                            target_window.emit_to("main", "files-hover-stopped", ()).unwrap();
                            target_window.emit_to("main", "sprite-changed", get_sprite(current_sprite)).unwrap();
                        }
                        _ => {
                            target_window.emit_to("main", "files-hover-stopped", ()).unwrap();
                        },
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_current_sprite(state: tauri::State<ApplicationState>) -> String {
    let symbols = state.current_sprite.read().unwrap().clone();

    let mut string = vec!["<svg xmlns=\"http://www.w3.org/2000/svg\"><defs>".to_string()];

    for symbol in symbols {
        string.push(symbol.to_string());
    }

    string.push("</defs></svg>".to_string());
    string.join("")
}

fn get_sprite(symbols: Vec<SvgSymbol>) -> String {
    let mut string = vec!["<svg xmlns=\"http://www.w3.org/2000/svg\"><defs>".to_string()];

    for symbol in symbols {
        string.push(symbol.to_string());
    }

    string.push("</defs></svg>".to_string());
    string.join("")
}
