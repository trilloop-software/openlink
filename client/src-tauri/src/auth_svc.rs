use tauri::{command, State};

use shared::{login::LoginCredentials, remote_conn_packet::*};
use super::remote_conn_svc::*;

#[command]
pub async fn check_auth(token: State<'_, super::Token>) -> Result<bool, bool> {
    let token = s!(&*token.0.lock().await);

    if token != "" {
        Ok(true)
    } else {
        Err(false)
    }
}

#[command]
pub async fn login(username: String, password: String, conn_state: State<'_, super::Connection>, token: State<'_, super::Token>) -> Result<String, String> {
    // ensure valid connection to pod computer
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let creds = LoginCredentials {
        username,
        password
    };

    let creds = serde_json::to_string(&creds).unwrap();

    let data = match send_(&conn, RemotePacket::new(1, vec![creds])).await {
        Ok(p) => p,
        Err(e) => return Err(s![e])
    };

    if data.cmd_type == 0 {
        return Err(s![data.payload[0]])
    } else {
        *token.0.lock().await = data.token.clone();
    }

    // return result to vue frontend
    Ok(data.payload[0].clone())
}

#[command]
pub async fn logout(token: State<'_, super::Token>) -> String {
    // delete the auth token
    *token.0.lock().await = s!("");

    s!("Logged out")
}