#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#[macro_use]
mod macros;

mod api;
use api::*;

#[derive(Default)]
pub struct Connection(tauri::async_runtime::Mutex<Option<quinn::Connection>>);
pub struct Token(tauri::async_runtime::Mutex<String>);

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .manage(Connection(Default::default()))
    .manage(Token(Default::default()))
    .invoke_handler(tauri::generate_handler![      
      auth::check_auth,
      auth::login,
      auth::logout,
      controls::launch,
      controls::set_destination,
      controls::stop,
      link::add_device,
      link::get_device_list,
      link::get_pod_state,
      link::lock_devices,
      link::remove_device,
      link::update_device,
      link::unlock_devices,
      remote_conn::check_conn,
      remote_conn::connect,
      remote_conn::disconnect,
      telemetry::get_telemetry,
      users::add_user,
      users::get_user_list,
      users::remove_user,
      users::update_user_group,
      users::update_user_password,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
