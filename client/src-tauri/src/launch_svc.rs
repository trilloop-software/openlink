use tauri::{command};

use super::{packet::*,remote_conn_svc::*};

#[command]
pub async fn launch() -> String{
    
    let my_packet = Packet {
        packet_id: s!["OPENLINK"],
        version: 1,
        cmd_type: 69,
        timestamp: std::time::SystemTime::now(),
        payload: vec!["Launch command from Client".to_string()]
    };

    println!("Data sent");
    let data = send_packet(my_packet).await;

    data.payload[0].clone()
}

async fn send_packet(my_packet: Packet) -> Packet {

    let data = send("127.0.0.1:6007".parse().unwrap(), my_packet).await;
    let data = data.unwrap();
    data
}
