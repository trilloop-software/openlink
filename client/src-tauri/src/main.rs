#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod cmd;
use cmd::{test};
use cmd::{ping_device};
use cmd::{add_device};

mod api_svc;
use api_svc::{get_device_list};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      test,
      get_device_list,
      ping_device,
      add_device,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
