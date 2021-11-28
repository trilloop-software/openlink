use tauri::{command};

#[command]
pub fn test() -> String {
    "Message passing works!".to_string()
}
