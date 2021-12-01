#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod cmd;
use cmd::{test, stop, emergency_stop, launch, set_destination};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      test, set_destination, launch, stop, emergency_stop
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
