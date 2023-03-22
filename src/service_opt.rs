use crate::SERVICE_LIABLE;
use clap::{Parser, Subcommand};
use service_manager::*;
use std::ffi::OsString;
use std::path::PathBuf;

pub fn service() -> anyhow::Result<Option<PathBuf>> {
    let current_exe = match std::env::current_exe() {
        Ok(path) => path,
        Err(err) => panic!("current_exe_path get error:{err:?}"),
    };

    match OptArgs::parse() {
        OptArgs::Exec { config } => Ok(Some(config)),
        OptArgs::Create => Ok(None),
        OptArgs::Service(ServiceArgs::Install { config }) => {
            let label: ServiceLabel = SERVICE_LIABLE.parse().unwrap();
            let manager = <dyn ServiceManager>::native()?;
            manager
                .install(ServiceInstallCtx {
                    label,
                    program: current_exe,
                    args: vec![OsString::from("exec"), OsString::from(config)],
                })
                .expect("Failed to install");
            println!("install success");
            Ok(None)
        }
        OptArgs::Service(ServiceArgs::Uninstall) => {
            let label: ServiceLabel = SERVICE_LIABLE.parse().unwrap();
            let manager = <dyn ServiceManager>::native()?;
            manager
                .uninstall(ServiceUninstallCtx { label })
                .expect("Failed to uninstall");
            println!("uninstall success");
            Ok(None)
        }
        OptArgs::Service(ServiceArgs::Start) => {
            let label: ServiceLabel = SERVICE_LIABLE.parse().unwrap();
            let manager = <dyn ServiceManager>::native()?;
            manager
                .start(ServiceStartCtx { label })
                .expect("Failed to start");
            println!("start success");
            Ok(None)
        }
        OptArgs::Service(ServiceArgs::Stop) => {
            let label: ServiceLabel = SERVICE_LIABLE.parse().unwrap();
            let manager = <dyn ServiceManager>::native()?;
            manager
                .stop(ServiceStopCtx { label })
                .expect("Failed to stop");
            println!("stop success");
            Ok(None)
        }
        OptArgs::Service(ServiceArgs::Restart) => {
            let label: ServiceLabel = SERVICE_LIABLE.parse().unwrap();
            let manager = <dyn ServiceManager>::native()?;

            manager
                .stop(ServiceStopCtx {
                    label: label.clone(),
                })
                .expect("Failed to stop");

            manager
                .start(ServiceStartCtx { label })
                .expect("Failed to start");

            println!("restart success");
            Ok(None)
        }
    }
}

#[derive(Parser)]
#[clap(name = "test service")]
enum OptArgs {
    Exec {
        #[arg(value_parser, default_value = "config.json")]
        config: PathBuf,
    },
    Create,
    #[command(subcommand)]
    Service(ServiceArgs),
}

#[derive(Subcommand)]
enum ServiceArgs {
    Install {
        #[arg(value_parser, default_value = "config.json")]
        config: String,
    },
    Start,
    Stop,
    Restart,
    Uninstall,
}
