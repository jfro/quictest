use crate::verifier::ServerVerifier;
use anyhow::Result;
use futures_util::StreamExt;
use quinn::{Endpoint, NewConnection};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

pub async fn client_main() -> Result<()> {
    let (cert, pkey) = crate::tls::generate_cert()?;
    let client_crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(ServerVerifier::new()))
        .with_single_cert(vec![cert], pkey)?;
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse().unwrap())?;
    let mut config = quinn::ClientConfig::new(Arc::new(client_crypto));
    Arc::get_mut(&mut config.transport)
        .unwrap()
        .keep_alive_interval(Some(Duration::from_secs(3)))
        .datagram_send_buffer_size(1024 * 1024 * 2)
        .datagram_receive_buffer_size(Some(1024 * 1024 * 2));
    endpoint.set_default_client_config(config);
    let conn = endpoint.connect("127.0.0.1:4321".parse()?, "localhost")?;
    let NewConnection {
        // connection,
        mut bi_streams,
        mut datagrams,
        ..
    } = conn.await?;

    info!("Spinning out datagram receiver");
    tokio::spawn(async move {
        let mut timer = tokio::time::interval(Duration::from_secs(2));
        let mut count = 0usize;
        let mut bytes = 0usize;
        loop {
            tokio::select! {
                _ = timer.tick() => {
                    info!("{} packets received, {} bytes", count, bytes);
                }
                Some(Ok(dgram)) = datagrams.next() => {
                    // info!("{} bytes received", dgram.len());
                    count += 1;
                    bytes += dgram.len();
                }
            }
        }
    });

    while let Some(stream) = bi_streams.next().await {
        match stream {
            Ok((mut send, mut recv)) => {
                info!("Got a new stream");
                send.write_all(b"hello").await?;
                tokio::spawn(async move {
                    let mut buf = [0u8; 4096];
                    while let Ok(Some(d)) = recv.read(&mut buf).await {
                        info!("{:?} bytes read", d);
                    }
                    info!("Stream finished");
                });
            }
            Err(e) => {
                error!("Error with incoming stream: {}", e);
                break;
            }
        }
    }

    Ok(())
}
