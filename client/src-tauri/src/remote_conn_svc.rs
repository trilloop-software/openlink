use anyhow::{anyhow, Result};
use quinn::{ClientConfig, Endpoint};
use std::{sync::Arc, net::SocketAddr};

use super::packet::*;

pub async fn send(server_addr: SocketAddr, pkt: Packet) -> Result<Packet> {
    let client_cfg = configure_client();

    let mut endpoint = Endpoint::client("0.0.0.0:0".parse().unwrap())?;    
    endpoint.set_default_client_config(client_cfg);

    let req = encode(pkt);

    let new_conn = endpoint
        .connect(server_addr, "localhost")?
        .await
        .map_err(|e| anyhow!("failed to connect: {}", e))?;

    let quinn::NewConnection { connection: conn, .. } = new_conn;

    let (mut send, recv) = conn
        .open_bi()
        .await
        .map_err(|e| anyhow!("failed to open stream: {}", e))?;

    send.write_all(&req)
        .await
        .map_err(|e| anyhow!("failed to send request: {}", e))?;

    send.finish()
        .await
        .map_err(|e| anyhow!("failed to shutdown stream: {}", e))?;

    let resp = recv
        .read_to_end(usize::max_value())
        .await
        .map_err(|e| anyhow!("failed to read response: {}", e))?;

    conn.close(0u32.into(), b"done");
    endpoint.wait_idle().await;

    Ok(decode(resp))
}

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

fn configure_client() -> ClientConfig {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    ClientConfig::new(Arc::new(crypto))
}
