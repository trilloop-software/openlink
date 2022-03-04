use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String
}
