use tauri::AppHandle;
use tauri::api::dialog::FileDialogBuilder;

#[cfg(target_os = "windows")]
const SAVE: &str = "CTRL + S";

#[cfg(target_os = "macos")]
const SAVE: &str = "CMD + S";

pub fn register_keybindings(app_handle: AppHandle) {
    // let app_state = app_handle.state::<ApplicationState>();

    // let res = app_handle.global_shortcut_manager()
    //     .register(SAVE, move || {
    //         FileDialogBuilder::new()
    //             .set_title("Save sprite")
    //             .add_filter("svg", &["svg"])
    //             .set_file_name("sprite.svg")
    //             .save_file(|file_path| {
    //                 let Some(file_path) = file_path else {
    //                     return
    //                 };
    //                 println!("File path: {:?}", file_path);
    //
    //                 // *app_state.file_path.write().unwrap() = Some(file_path.to_str().unwrap().to_string());
    //             });
    //     });
    //
    // if let Err(e) = res {
    //     println!("Error registering global shortcut: {}", e);
    // }
}
