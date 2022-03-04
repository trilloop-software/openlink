#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#[macro_use]
mod macros;

mod cmd;
use cmd::{test, ping_device, set_destination};

mod api_svc;
use api_svc::{lock_devices, unlock_devices, add_device, get_device_list, remove_device, update_device, get_pod_state};

mod auth_svc;

mod brake_svc;
use brake_svc::{stop};

mod emerg_svc;
use emerg_svc::{emergency_stop};

mod launch_svc;
use launch_svc::{launch};

mod remote_conn_svc;

#[derive(Default)]
pub struct Connection(tauri::async_runtime::Mutex<Option<quinn::Connection>>);
pub struct Token(tauri::async_runtime::Mutex<String>);

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .manage(Connection(Default::default()))
    .manage(Token(Default::default()))
    .invoke_handler(tauri::generate_handler![
      test,
      get_pod_state,
      lock_devices,
      unlock_devices,
      add_device,
      get_device_list,
      remove_device,
      update_device,
      ping_device,
      stop,
      emergency_stop,
      set_destination,
      launch,
      remote_conn_svc::connect,
      auth_svc::login,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
