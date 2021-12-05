#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod cmd;
use cmd::{test};
use cmd::{test_ping_device};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      test,
      test_ping_device,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
