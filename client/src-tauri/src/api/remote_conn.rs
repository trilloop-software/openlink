use anyhow::Result;
use quinn::{ClientConfig, Endpoint};
use std::sync::Arc;
use tauri::{command, State};

use shared::{remote_conn_packet::*};
use super::super::Connection;

#[command]
pub async fn check_conn(conn_state: State<'_, Connection>) -> Result<bool, bool> {
    let conn = &*conn_state.0.lock().await;

    match conn {
        Some(_) => Ok(true),
        None => Err(false)
    }
}

/// establishes persistent QUIC connection to pod computer and stores in a tauri state if successful
/// returns a boolean to the frontend representing connection state
/// match statements are used to adequately handle errors rather than crashing the tauri thread
#[command]
pub async fn connect(addr: String, conn_state: State<'_, Connection>) -> Result<String, String> {
    let server_addr = addr.parse().unwrap();

    let client_cfg = configure_client();
    let endpoint = Endpoint::client("0.0.0.0:0".parse().unwrap());
    let mut endpoint = match endpoint {
        Ok(endpoint) => endpoint,
        Err(_) => return Err(s!["Failed to bind local network interface"])
    };

    endpoint.set_default_client_config(client_cfg);

    let new_conn = endpoint
        .connect(server_addr, "localhost");

    let new_conn = match new_conn {
        Ok(new_conn) => new_conn,
        Err(_) => return Err(s!["Failed to start connection to Pod Computer"])
    };

    let new_conn = match new_conn.await {
        Ok(new_conn) => new_conn,
        Err(_) => return Err(s!["Failed to open connection to Pod Computer"])
    };

    let quinn::NewConnection { connection: conn, .. } = new_conn;

    *conn_state.0.lock().await = Some(conn);

    Ok(s!["Connected"])
}

#[command]
pub async fn disconnect(conn_state: State<'_, Connection>) -> Result<String, String> {
    *conn_state.0.lock().await = None;

    Ok(s!("Disconnected"))
}

/// utilizes QUIC connection to send and receive packets from the pod computer
/// returns a result packet with either a valid payload or an error
/// match statements are used to adequately handle errors rather than crashing the tauri thread
pub async fn send(conn: &quinn::Connection, pkt: RemotePacket) -> Result<RemotePacket, String> {
    let (mut send, recv) = match conn.open_bi().await {
        Ok((send, recv)) => (send, recv),
        Err(_) => return Err(s!["Failed to open send and receive streams"])
    };

    let req = encode(pkt);
    
    match send.write_all(&req).await {
        Ok(()) => (),
        Err(_) => return Err(s!["Failed to send request"])
    };
    
    match send.finish().await {
        Ok(()) => (),
        Err(_) => return Err(s!["Failed to shutdown stream"])
    };

    let resp = match recv.read_to_end(usize::max_value()).await {
        
        Ok(resp) => resp,
        Err(_) => return Err(s!["Failed to read response"])
    };
    
    Ok(decode(resp))
}

/// Skipping server certificate verification to simplify development for now
/// TODO: actually implement use of valid certs on server/client
struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

/// Configure client to ignore server certificate verification
fn configure_client() -> ClientConfig {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    ClientConfig::new(Arc::new(crypto))
}
