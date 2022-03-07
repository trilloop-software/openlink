use tauri::{command, State};

use shared::{launch::*, remote_conn_packet::*};
use super::{super::{Connection, Token}, remote_conn::*};

#[command]
pub async fn launch(conn_state: State<'_, Connection>, token: State<'_, Token>) -> Result<String, String> {
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let data = match send(&conn, RemotePacket::new_with_auth(69,vec![s!("")], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s!(e))
    };

    Ok(data.payload[0].clone())
}

#[command]
pub async fn set_destination(params: String, conn_state: State<'_, Connection>, token: State<'_, Token>) -> Result<String, String> {
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let params: LaunchParams = match serde_json::from_str(&params) {
        Ok(p) => p,
        Err(e) => return Err(s!(e))
    };

    let params = serde_json::to_string(&params).unwrap();

    let data = match send(&conn, RemotePacket::new_with_auth(68, vec![s!(params)], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s!(e))
    };

    Ok(data.payload[0].clone())
}

#[command]
pub async fn stop(conn_state: State<'_, Connection>, token: State<'_, Token>) -> Result<String, String> {
    let conn = &*conn_state.0.lock().await;
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let data = match send(&conn, RemotePacket::new_with_auth(99,vec![s!("")], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s!(e))
    };

    Ok(data.payload[0].clone())
}