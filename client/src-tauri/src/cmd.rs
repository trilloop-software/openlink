use tauri::{command};

#[command]
pub fn test() -> String {
    "Message passing works!".to_string()
}
#[command]
pub fn stop() -> String //in the future, we will want to actually send a message to the backend here.
{
    "Stop command sent".to_string()
}
#[command]
pub fn emergency_stop() -> String //same as above.
{
    "Emergency stop command sent".to_string()
}
#[command]
pub fn launch() -> String 
{
    "Start command sent".to_string()
}
#[command]
pub fn set_destination() -> String 
{
    "Set course destionation command sent".to_string()
}