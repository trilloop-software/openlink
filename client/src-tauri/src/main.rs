#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#[macro_use]
mod macros;

mod cmd;
use cmd::{test, ping_device, launch, set_destination};

mod api_svc;
use api_svc::{add_device, get_device_list, remove_device, update_device};

mod brake_svc;
use brake_svc::{stop};

mod emerg_svc;
use emerg_svc::{emergency_stop};

mod device;
mod packet;
mod remote_conn_svc;

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      test,
      add_device,
      get_device_list,
      remove_device,
      update_device,
      ping_device,
      stop,
      emergency_stop,
      set_destination,
      launch
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
