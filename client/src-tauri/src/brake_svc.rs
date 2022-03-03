use tauri::{command};

use shared::{remote_conn_packet::*};
use super::{remote_conn_svc::*};
#[command]
pub async fn stop() -> String{
    let pkt = RemotePacket::new(99,vec![1.to_string()]);
    println!("Data sent");
    let data = send_packet(pkt).await;
    // return result to vue frontend
    data.payload[0].clone()
}

async fn send_packet(pkt: RemotePacket) -> RemotePacket {
    // hardcoding server_addr temporarily, following Matt's example in api_svc
    let data = send("127.0.0.1:6007".parse().unwrap(), pkt).await;
    let data = data.unwrap();
    data
}