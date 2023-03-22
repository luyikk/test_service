use serde::Deserialize;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub listen: String,
}

impl TryFrom<PathBuf> for Config {
    type Error = io::Error;

    fn try_from(config_file: PathBuf) -> Result<Self, Self::Error> {
        if config_file.exists() {
            Ok(serde_json::from_str(&std::fs::read_to_string(config_file)?)
                .map_err(|er| io::Error::new(ErrorKind::Other, er))?)
        } else {
            let mut current_exec_path = crate::io::get_current_exec_path()?;

            current_exec_path.push(config_file);

            if current_exec_path.exists() {
                Ok(
                    serde_json::from_str(&std::fs::read_to_string(current_exec_path)?)
                        .map_err(|er| io::Error::new(ErrorKind::Other, er))?,
                )
            } else {
                Err(io::Error::new(
                    ErrorKind::NotFound,
                    format!("not found config:{current_exec_path:?}"),
                ))
            }
        }
    }
}
