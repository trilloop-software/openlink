use tauri::{command};

#[command]
pub fn test() -> String {
    "Message passing works!".to_string()
}

#[tauri::command]
pub fn test_ping_device() -> String{
    "Pinging device works!".to_string()
}
