
use tauri::{command};

#[command]
pub fn test() -> String {
    "Message passing works!".to_string()
}

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub ip_address: String,
}

#[tauri::command]
pub fn get_device_list() -> String{

    //return a string for now
    //parsing the list of devices into a packet is for sprint 2
    return "Device List on Backend: Placeholder Battery, Placeholder Inverter".to_string()
}

#[tauri::command]
pub fn ping_device(name:&str) -> String{

    let mut owned_string: String = "Ping Request received for ".to_owned();
    let borrowed_string: &str = name;
    
    owned_string.push_str(borrowed_string);

    //return a string for now
    //parsing the actual request
    return owned_string;
}