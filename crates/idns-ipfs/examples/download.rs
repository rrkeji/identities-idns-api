use anyhow::Result;
use ipfs_idns_api::download;

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("start....");

    let version = "v0.15.0";
    let install_path = "/Users/suhs/temp/";

    download(version, install_path).await?;

    Ok(())
}
