use anyhow::Result;
use serde_json;
use tauri::{command, State};

use super::{device::*, packet::*, remote_conn_svc::*};

#[command]
pub async fn lock_devices(conn_state: State<'_, super::Connection>) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let data = match send_(&conn, Packet::new(63, vec![s![""]])).await {
        Ok(p) => p,
        Err(e) => return Err(s![e])
    };

    // return result to vue frontend
    Ok(data.payload[0].clone())
}

#[command]
pub async fn unlock_devices(conn_state: State<'_, super::Connection>) -> Result<String, String> {
    // lock tauri state and ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    // send new device to pod computer
    let data = match send_(&conn, Packet::new(62, vec![s![""]])).await {
        Ok(p) => p,
        Err(e) => return Err(s![e])
    };

    // send success response to frontend
    Ok(data.payload[0].clone())
}

#[command]
pub async fn add_device(dev: String, conn_state: State<'_, super::Connection>) -> Result<String, String> {
    // lock tauri state and ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    // verify device has valid properties
    let dev: Device = match serde_json::from_str(&dev) {
        Ok(d) => d,
        Err(e) => return Err(s![e])
    };

    let dev = serde_json::to_string(&dev).unwrap();

    // send new device to pod computer
    let data = match send_(&conn, Packet::new(33, vec![dev])).await {
        Ok(p) => p,
        Err(e) => return Err(s![e])
    };

    // send success response to frontend
    Ok(data.payload[0].clone())
}

#[command]
pub async fn get_device_list(conn_state: State<'_, super::Connection>) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let data = match send_(&conn, Packet::new(32, vec![s![""]])).await {
        Ok(p) => p,
        Err(e) => return Err(s![e])
    };

    // return result to vue frontend
    Ok(data.payload[0].clone())
}


#[command]
pub async fn remove_device(dev: String, conn_state: State<'_, super::Connection>) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    // verify device has valid properties
    let dev: Device = match serde_json::from_str(&dev) {
        Ok(d) => d,
        Err(e) => return Err(s![e])
    };

    let dev = serde_json::to_string(&dev).unwrap();

    // send device to remove from pod computer
    let data = match send_(&conn, Packet::new(35, vec![dev])).await {
        Ok(p) => p,
        Err(e) => return Err(s![e])
    };

    // send success response to frontend
    Ok(data.payload[0].clone())
}

#[command]
pub async fn update_device(dev: String, conn_state: State<'_, super::Connection>) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    // verify device has valid properties
    let dev: Device = match serde_json::from_str(&dev) {
        Ok(d) => d,
        Err(e) => return Err(s![e])
    };

    let dev = serde_json::to_string(&dev).unwrap();

    // send device to update on pod computer
    let data = match send_(&conn, Packet::new(34, vec![dev])).await {
        Ok(p) => p,
        Err(e) => return Err(s![e])
    };

    // send success response to frontend
    Ok(data.payload[0].clone())
}
