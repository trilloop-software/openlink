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

#[command]
pub fn set_destination() -> String 
{
    "Set course destionation command sent".to_string()
}
