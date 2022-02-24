use tauri::{command};

use super::{packet::*, remote_conn_svc::*};
#[command]
pub async fn stop() -> String{
    let pkt = Packet {
        packet_id: s!["OPENLINK"],
        version: 1,
        cmd_type: 99,
        timestamp: std::time::SystemTime::now(),
        payload: vec![1.to_string()]
    };
    println!("Data sent");
    let data = send_packet(pkt).await;
    // return result to vue frontend
    data.payload[0].clone()
}

async fn send_packet(pkt: Packet) -> Packet {
    // hardcoding server_addr temporarily, following Matt's example in api_svc
    let data = send("127.0.0.1:6007".parse().unwrap(), pkt).await;
    let data = data.unwrap();
    data
}