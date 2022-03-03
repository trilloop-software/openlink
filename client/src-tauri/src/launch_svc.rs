use tauri::{command};

use shared::{remote_conn_packet::*};
use super::{remote_conn_svc::*};

#[command]
pub async fn launch(/*my_time: String*/) -> String{
    //println!("{}" ,my_time);
    let my_packet = RemotePacket::new(69,vec!["Launch command from Client".to_string()]);

    println!("Data sent");
    let data = send_packet(my_packet).await;

    data.payload[0].clone()
}

async fn send_packet(my_packet: RemotePacket) -> RemotePacket {

    let data = send("127.0.0.1:6007".parse().unwrap(), my_packet).await;
    let data = data.unwrap();
    data
}
