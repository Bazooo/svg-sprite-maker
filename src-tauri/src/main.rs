// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use svg_sprite_parser::parser::{get_svg_type_from_file, SvgType};
use tauri::{FileDropEvent, Manager, WindowEvent};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let binding = app.windows();
            let main_window = binding.get("main").unwrap();
            let target_window = main_window.clone();

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
                            let x = get_svg_type_from_file(dropped_files.first().unwrap());
                        }
                        _ => {
                            target_window.emit_to("main", "file-drop-cancelled", ()).unwrap();
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
