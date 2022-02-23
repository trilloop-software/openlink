use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet_Discovery {
    pub packet_id: String,
    pub version: u8,
    pub cmd_type: u8,
    pub timestamp: SystemTime,
    pub payload: Vec<String>
}

impl Packet_Discovery {
    pub fn new(cmd_type: u8, payload: Vec<String>) -> Self {
        Self {
            packet_id: s!["OPENLINK"],
            version: 1,
            cmd_type: cmd_type,
            timestamp: std::time::SystemTime::now(),
            payload: payload
        }
    }
}

pub fn decode(pkt: Vec<u8>) -> Packet_Discovery {
    deserialize(&pkt[..]).unwrap()
}

pub fn encode(pkt: Packet_Discovery) -> Vec<u8> {
    serialize(&pkt).unwrap()
}
