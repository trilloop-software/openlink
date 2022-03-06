use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub packet_id: String,
    pub version: u8,
    pub cmd_type: u8,
    pub timestamp: SystemTime,
    pub target_cmd_code: u8, //command to be sent to the device in the payload
    pub payload: Vec<String>
}

impl Packet {
    pub fn new(cmd_type: u8, payload: Vec<String>) -> Self {
        Self {
            packet_id: s!["OPENLINK"],
            version: 1,
            cmd_type: cmd_type,
            timestamp: std::time::SystemTime::now(),
            target_cmd_code:0,
            payload: payload
        }
    }
}

pub fn decode(pkt: Vec<u8>) -> Packet {
    deserialize(&pkt[..]).unwrap()
}

pub fn encode(pkt: Packet) -> Vec<u8> {
    serialize(&pkt).unwrap()
}
