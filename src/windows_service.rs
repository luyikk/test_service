use std::ffi::OsString;
use std::io;
use std::time::Duration;
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher, Result,
};

const SERVICE_NAME: &str = crate::SERVICE_LIABLE;
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

pub fn run() -> io::Result<()> {
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)
        .map_err(|x| io::Error::new(io::ErrorKind::Other, x))
}

define_windows_service!(ffi_service_main, service_main);

pub fn service_main(arguments: Vec<OsString>) {
    log::info!("service arguments:{arguments:?}");
    if let Err(err)=run_service(){
        log::error!("run service error:{err:?}");
    }
}

fn run_service() -> Result<()> {
    log::info!("Starting windows service for {SERVICE_NAME}");

    // Create a channel to be able to poll a stop event from the service worker loop.
    let (shutdown_tx, shutdown_rx) = std::sync::mpsc::channel();
    // Define system service event handler that will be receiving service events.
    let event_handler = {
        let shutdown_tx = shutdown_tx.clone();
        move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                // Notifies a service to report its current status information to the service
                // control manager. Always return NoError even if not implemented.
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

                // Handle stop
                ServiceControl::Stop => {
                    if shutdown_tx.send(()).is_err() {
                        log::error!("shutdown_tx send error");
                    };
                    ServiceControlHandlerResult::NoError
                }

                _ => ServiceControlHandlerResult::NotImplemented,
            }
        }
    };

    log::info!("Registering service control handler for {SERVICE_NAME}");
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    // Tell the system that service is running
    log::info!("Setting service status as running for {SERVICE_NAME}");

    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    log::info!("Spawning CLI thread for {SERVICE_NAME}");

    std::thread::spawn(|| {
        match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(runtime) => {
                if let Err(err) = runtime.block_on(crate::start(None)) {
                    log::error!("tokio error:{err}")
                }
            }
            Err(err) => {
                log::error!("tokio runtime build error:{err}")
            }
        }
    });

    loop {
        match shutdown_rx.recv_timeout(Duration::from_millis(100)) {
            // Break the loop either upon stop or channel disconnect
            Ok(_) | Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,

            // Continue work if no events were received within the timeout
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => (),
        };
    }

    // Tell the system that service has stopped.
    log::info!("Setting service status as stopped for {SERVICE_NAME}");
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    crate::LOGGER_HANDLER.get().unwrap().shutdown();

    Ok(())
}
