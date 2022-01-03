use tauri::{command};
use std::fs; // TEMPORARY TO READ FROM JSON FILE

#[command]
pub fn get_device_list() -> String {
    let data = fs::read_to_string("src/temp_data/device_list.json").expect("Unable to get device list.");

    data
}
