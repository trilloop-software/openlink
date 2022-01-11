use serde_json;
use tauri::{command};

use super::{device::*, packet::*, remote_conn_svc::*};

// -----------------
// COMMAND FUNCTIONS
// -----------------
#[command]
pub async fn add_device(dev: String) -> String {
    // parse device received from vue frontend to ensure parameters are valid
    let dev: Device = match serde_json::from_str(&dev) {
        Ok(s) => s,
        Err(e) => return s![e]
    };

    // convert back to string for payload
    let dev = serde_json::to_string(&dev).unwrap();

    // construct add_device packet with the device as payload
    let pkt = Packet {
        packet_id: s!["OPENLINK"],
        version: 1,
        cmd_type: 33,
        timestamp: std::time::SystemTime::now(),
        payload: vec![dev]
    };

    let data = send_packet(pkt).await;

    // return result to vue frontend
    data.payload[0].clone()
}

#[command]
pub async fn get_device_list() -> String {
    // construct the get_device_list packet with empty payload
    let pkt = Packet {
        packet_id: s!["OPENLINK"],
        version: 1,
        cmd_type: 32,
        timestamp: std::time::SystemTime::now(),
        payload: vec![s![""]]
    };

    let data = send_packet(pkt).await;

    // return result to vue frontend
    data.payload[0].clone()
}

#[command]
pub async fn remove_device(dev: String) -> String {
    // parse device received from vue frontend to ensure parameters are valid
    let dev: Device = match serde_json::from_str(&dev) {
        Ok(s) => s,
        Err(e) => return s![e]
    };

    // convert back to string for packet
    let dev = serde_json::to_string(&dev).unwrap();

    // construct remove_device packet with the device as payload
    let pkt = Packet {
        packet_id: s!["OPENLINK"],
        version: 1,
        cmd_type: 35,
        timestamp: std::time::SystemTime::now(),
        payload: vec![dev]
    };

    let data = send_packet(pkt).await;

    // return result to vue frontend
    data.payload[0].clone()
}

#[command]
pub async fn update_device(dev: String) -> String {
    // parse device received from vue frontend to ensure parameters are valid
    let dev: Device = match serde_json::from_str(&dev) {
        Ok(s) => s,
        Err(e) => return s![e]
    };

    // convert back to string for packet
    let dev = serde_json::to_string(&dev).unwrap();
    
    // construct update_device packet with the device as payload
    let pkt = Packet {
        packet_id: s!["OPENLINK"],
        version: 1,
        cmd_type: 34,
        timestamp: std::time::SystemTime::now(),
        payload: vec![dev]
    };

    let data = send_packet(pkt).await;

    // return result to vue frontend
    data.payload[0].clone()
}

// ----------------
// HELPER FUNCTIONS
// ----------------
async fn send_packet(pkt: Packet) -> Packet {
    // hardcoding server_addr temporarily
    let data = send("127.0.0.1:6007".parse().unwrap(), pkt).await;
    let data = data.unwrap();

    data
}
