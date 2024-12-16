use thiserror::Error;

#[derive(Error, Debug)]
pub enum RSComError {
    #[error("Failed to list available serial ports")]
    ListPortsError(#[from] tokio_serial::Error),
}