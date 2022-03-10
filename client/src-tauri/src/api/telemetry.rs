use tauri::{command, State};

use shared::remote_conn_packet::*;
use super::{super::{Connection, Token}, remote_conn::*};

#[command]
pub async fn get_telemetry(conn_state: State<'_, Connection>, token: State<'_, Token>) -> Result<Vec<String>, String> {
    //println!("LOCKING");
    let conn = &*conn_state.0.lock().await;
    //println!("unLOCKING");
    let conn = conn_test!(conn);

    let token = s!(&*token.0.lock().await);

    let data = match send(&conn, RemotePacket::new_with_auth(128, vec![s!("")], token)).await {
        Ok(p) => p,
        Err(e) => return Err(s!(e))
    };

    if data.cmd_type == 0 {
        return Err(s!(data.payload[0]))
    }

    Ok(data.payload.clone())
}
