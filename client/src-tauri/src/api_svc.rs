use tauri::{command};

use super::{packet::*, remote_conn_svc::*};

#[command]
pub async fn get_device_list() -> String {
    let pkt = Packet {
        packet_id: "OPENLINK".to_string(),
        version: 1,
        cmd_type: 10,
        timestamp: std::time::SystemTime::now(),
        payload: vec!["test".to_string()]
    };

    let data = send("127.0.0.1:6007".parse().unwrap(), pkt).await;
    let data = data.unwrap();

    data.payload[0].clone()
}
