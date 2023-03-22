mod config;
mod io;
mod logger;
mod service_opt;
#[cfg(windows)]
mod windows_service;

use crate::config::Config;
use crate::service_opt::service;
use logger::*;
use std::path::PathBuf;
use std::time::Duration;

const SERVICE_LIABLE: &str = "com.test.service";

#[cfg(unix)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Some(config) = service()? {
        install_logger()?;
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
