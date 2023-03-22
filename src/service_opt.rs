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
        OptArgs::Create => {
            let config = include_str!("../config.json");
            let mut path = crate::io::get_current_exec_path()?;
            path.push("config.json");
            std::fs::write(path, config)?;
            Ok(None)
        }
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
            println!("service install success");
            Ok(None)
        }
        OptArgs::Service(ServiceArgs::Uninstall) => {
            let label: ServiceLabel = SERVICE_LIABLE.parse().unwrap();
            let manager = <dyn ServiceManager>::native()?;
            manager
                .uninstall(ServiceUninstallCtx { label })
                .expect("Failed to uninstall");
            println!("service uninstall success");
            Ok(None)
        }
        OptArgs::Service(ServiceArgs::Start) => {
            let label: ServiceLabel = SERVICE_LIABLE.parse().unwrap();
            let manager = <dyn ServiceManager>::native()?;
            manager
                .start(ServiceStartCtx { label })
                .expect("Failed to start");
            println!("service start success");
            Ok(None)
        }
        OptArgs::Service(ServiceArgs::Stop) => {
            let label: ServiceLabel = SERVICE_LIABLE.parse().unwrap();
            let manager = <dyn ServiceManager>::native()?;
            manager
                .stop(ServiceStopCtx { label })
                .expect("Failed to stop");
            println!("service stop success");
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

            println!("service restart success");
            Ok(None)
        }
    }
}

#[derive(Parser)]
#[clap(name = "test service")]
enum OptArgs {
    /// run service
    Exec {
        /// config path;(by default, read from the current exec path config.json)
        #[arg(value_parser, default_value = "config.json")]
        config: PathBuf,
    },
    /// create default config.json to current exec path
    Create,
    /// service manager
    #[command(subcommand)]
    Service(ServiceArgs),
}

#[derive(Subcommand)]
enum ServiceArgs {
    /// install service to system
    Install {
        #[arg(value_parser, default_value = "config.json")]
        config: String,
    },
    /// start service
    Start,
    /// stop service
    Stop,
    /// restart service
    Restart,
    /// uninstall service to system
    Uninstall,
}
