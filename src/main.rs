mod service_opt;
mod stdout_log;
#[cfg(windows)]
mod windows_service;

use std::path::PathBuf;
use std::time::Duration;

use crate::service_opt::service;

const SERVICE_LIABLE: &str = "com.test.service";

#[cfg(unix)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Some(config) = service()? {
        install_logger()?;
        start(Some(config)).await?;
    }
    Ok(())
}

#[cfg(windows)]
fn main() -> anyhow::Result<()> {
    if let Some(config) = service()? {
        install_logger()?;
        log::info!("start windows run");
        windows_service::run()?;
    }
    Ok(())
}

async fn start(config: Option<PathBuf>) -> anyhow::Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        log::info!("config:{config:?}");
    }
    Ok(())
}

static LOGGER_HANDLER: tokio::sync::OnceCell<flexi_logger::LoggerHandle> =
    tokio::sync::OnceCell::const_new();

fn install_logger() -> anyhow::Result<()> {
    use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger, Naming, WriteMode};
    #[cfg(unix)]
    let logger = Logger::try_with_str("trace, mio=error")?
        .log_to_file_and_writer(
            FileSpec::default()
                .directory("logs")
                .suppress_timestamp()
                .suffix("log"),
            Box::new(stdout_log::StdErrLog::new()),
        )
        .format(flexi_logger::opt_format)
        .rotate(
            Criterion::AgeOrSize(Age::Day, 1024 * 1024 * 5),
            Naming::Numbers,
            Cleanup::KeepLogFiles(200),
        )
        .print_message()
        .set_palette("196;190;2;4;8".into())
        .write_mode(WriteMode::Async)
        .start()?;

    #[cfg(windows)]
    let logger= {

        let mut logs = match std::env::current_exe(){
            Ok(path)=>{
                if let Some(current_exe_path)= path.parent(){
                    current_exe_path.to_path_buf()
                }else {
                    panic!("current_exe_path get error: is none");
                }
            },
            Err(err)=> panic!("current_exe_path get error:{err:?}")
        };

        logs.push("logs");
        println!("{:?}",logs);
        let logger = Logger::try_with_str("trace, sqlx = error,mio = error")?
            .log_to_file(FileSpec::default().directory(logs).suppress_timestamp().suffix("log"))
            .format(flexi_logger::opt_format)
            .rotate(Criterion::AgeOrSize(Age::Day, 1024 * 1024 * 5), Naming::Numbers, Cleanup::KeepLogFiles(200))
            .print_message()
            .write_mode(WriteMode::Async)
            .start()?;
        logger
    };


    LOGGER_HANDLER
        .set(logger)
        .map_err(|_| anyhow::anyhow!("logger set error"))?;

    Ok(())
}
