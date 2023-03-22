mod service;
use std::path::PathBuf;
use std::time::Duration;

const SERVICE_LIABLE: &str = "com.test.service";

#[cfg(unix)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Some(config) = service::service_opt::service()? {
        service::logger::install_logger()?;
        log::info!("start unix run");
        start(config).await?;
    }
    Ok(())
}

#[cfg(windows)]
fn main() -> anyhow::Result<()> {
    if let Some(config_file) = service::service_opt::service()? {
        service::windows_service::CONFIG_FILE.set(config_file)?;
        service::logger::install_logger()?;
        log::info!("start windows run");
        service::windows_service::run()?;
    }
    Ok(())
}

#[inline]
async fn start(config_file: PathBuf) -> anyhow::Result<()> {
    let config = service::config::Config::try_from(config_file)?;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        log::info!("config:{config:?}");
    }
    // Ok(())
}
