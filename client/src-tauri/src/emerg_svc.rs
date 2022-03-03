use tauri::{command};

use shared::{remote_conn_packet::*};
use super::{remote_conn_svc::*};
#[command]
pub async fn emergency_stop() -> String{

    //a stop command may have more parameters in the future, but right now it doesn't really need a payload
    let pkt = RemotePacket::new(255,vec!["Rip and tear, until it is done".to_string()]);
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