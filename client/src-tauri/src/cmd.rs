use tauri::{command};
#[command]
pub fn test() -> String {
    "Message passing works!".to_string()
}

#[command]
pub fn ping_device(name:&str) -> String{

    let mut owned_string: String = "Ping Request received for ".to_owned();
    let borrowed_string: &str = name;
    
    owned_string.push_str(borrowed_string);

    //return a string for now
    //parsing the actual request
    return owned_string;
}

/*#[tauri::command]
pub fn add_device(device_type:&str) -> String{

    let mut owned_string: String = "Received request to add new device (".to_owned();
    owned_string.push_str("Type: ");
    owned_string.push_str(device_type);
    owned_string.push_str(" )");

    //return a string for now
    //processing the packet is for sprint 2
    return owned_string;
}*/

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
