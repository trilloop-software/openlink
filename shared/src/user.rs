use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSecure {
    pub name: String,
    pub ugroup: u8
}

impl UserSecure {
    pub fn from_sql(name: String, ugroup: u8) -> Self {
        Self {
            name,
            ugroup
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRaw {
    pub name: String,
    pub pwd: String,
    pub ugroup: u8
}
