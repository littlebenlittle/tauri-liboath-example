#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_key, get_totp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn set_key(key: String) {
    // TODO
}

#[tauri::command]
fn get_totp() -> String {
    // TODO
    "lies!".into()
}
