#[cfg(windows)]
mod service;

use service::config::Config;
use service::logger::*;
use service::service_opt::service;
use service::windows_service;
use std::path::PathBuf;
use std::time::Duration;

const SERVICE_LIABLE: &str = "com.test.service";

#[cfg(unix)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Some(config) = service()? {
        install_logger()?;
        log::info!("start unix run");
        start(config).await?;
    }
    Ok(())
}

#[cfg(windows)]
fn main() -> anyhow::Result<()> {
    if let Some(config) = service()? {
        windows_service::CONFIG.set(config)?;
        install_logger()?;
        log::info!("start windows run");
        windows_service::run()?;
    }
    Ok(())
}

async fn start(config: PathBuf) -> anyhow::Result<()> {
    let config = Config::try_from(config)?;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        log::info!("config:{config:?}");
    }
    // Ok(())
}
