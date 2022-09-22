use anyhow::Result;
use tracing::level_filters::LevelFilter;

mod client;
mod server;
mod tls;
mod verifier;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(LevelFilter::INFO)
            .finish(),
    )
    .unwrap();
    let server_task = tokio::spawn(server::server_main());
    let client_task = tokio::spawn(client::client_main());
    client_task.await??;
    server_task.await??;
    Ok(())
}
