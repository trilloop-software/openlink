use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RemotePacket {
    pub packet_id: String,
    pub version: u8,
    pub cmd_type: u8,
    pub timestamp: SystemTime,
    pub target_cmd_code: u8, //command to be sent to the device in the payload
    pub payload: Vec<String>,
    pub token: String,
}

impl RemotePacket {
    pub fn new(cmd_type: u8, payload: Vec<String>) -> Self {
        Self {
            packet_id: s!["OPENLINK"],
            version: 1,
            cmd_type,
            timestamp: std::time::SystemTime::now(),
            target_cmd_code:0,
            payload,
            token: s![""]
        }
    }

    pub fn new_with_auth(cmd_type: u8, payload: Vec<String>, token: String) -> Self {
        Self {
            packet_id: s!["OPENLINK"],
            version: 1,
            cmd_type,
            timestamp: std::time::SystemTime::now(),
            target_cmd_code: 0,
            payload,
            token
        }
    }

    pub fn error(mut self, msg: String) -> Self {
        self.cmd_type = 0;
        self.payload[0] = msg;
        self
    }
}

pub fn decode(pkt: Vec<u8>) -> RemotePacket {
    deserialize(&pkt[..]).unwrap()
}

pub fn encode(pkt: RemotePacket) -> Vec<u8> {
    serialize(&pkt).unwrap()
}

