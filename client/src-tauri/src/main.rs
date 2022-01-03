#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod cmd;
use cmd::{test, ping_device, add_device, stop, emergency_stop};

mod api_svc;
use api_svc::{get_device_list};


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      test,
      get_device_list,
      ping_device,
      add_device,
      test,
      stop,
      emergency_stop
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
