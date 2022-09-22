use crate::verifier::ClientVerifier;
use anyhow::Result;
use futures_util::StreamExt;
use quinn::{Endpoint, NewConnection, ServerConfig};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

fn server_config() -> Result<ServerConfig> {
    let (cert, pkey) = crate::tls::generate_cert()?;
    let server_crypto = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_client_cert_verifier(Arc::new(ClientVerifier::new()))
        .with_single_cert(vec![cert], pkey)?;
    Ok(ServerConfig::with_crypto(Arc::new(server_crypto)))
}
pub async fn server_main() -> Result<()> {
    let config = server_config()?;
    let (_endpoint, mut incoming) = Endpoint::server(config, "127.0.0.1:4321".parse()?)?;

    while let Some(conn) = incoming.next().await {
        match conn.await {
            Ok(conn) => {
                tokio::spawn(handle_connection(conn));
            }
            Err(e) => {
                error!("Failed to fully accept connection: {}", e);
            }
        }
    }
    Ok(())
}

async fn handle_connection(conn: NewConnection) -> Result<()> {
    let NewConnection {
        connection,
        // datagrams,
        ..
    } = conn;
    let (mut send, mut recv) = connection.open_bi().await?;
    send.write_all(b"hello").await?;
    tokio::spawn(async move {
        info!("Starting datagram sender");
        let max = connection.max_datagram_size().unwrap_or(1024);
        let count = 1000;
        let mut bytes = 0;
        for _i in 0..count {
            let test = vec![0u8; max];

            match connection.send_datagram(test.into()) {
                Ok(_) => {
                    bytes += max;
                }
                Err(e) => {
                    error!("Failed to send: {}", e);
                }
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        info!("Sent {} packets, {} bytes", count, bytes);
    });
    let mut buf = [0u8; 4096];
    while let Ok(Some(bytes)) = recv.read(&mut buf).await {
        info!("{} bytes read", bytes);
    }
    info!("Done with connection");
    Ok(())
}
