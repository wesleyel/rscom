use std::fmt::Display;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio_serial::{available_ports, SerialPortInfo};

use crate::err::RSComError;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
pub enum Baudrate {
    Baud9600,
    Baud19200,
    Baud38400,
    Baud57600,
    Baud115200,
    Baud230400,
    Baud460800,
    Baud921600,
    Baud1000000,
}

impl Display for Baudrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u32::from(*self))
    }
}

impl From<Baudrate> for u32 {
    fn from(rate: Baudrate) -> Self {
        match rate {
            Baudrate::Baud9600 => 9600,
            Baudrate::Baud19200 => 19200,
            Baudrate::Baud38400 => 38400,
            Baudrate::Baud57600 => 57600,
            Baudrate::Baud115200 => 115200,
            Baudrate::Baud230400 => 230400,
            Baudrate::Baud460800 => 460800,
            Baudrate::Baud921600 => 921600,
            Baudrate::Baud1000000 => 1000000,
        }
    }
}

pub fn list_serial_ports() -> Result<Vec<SerialPortInfo>, RSComError> {
    let ports = available_ports()?;
    Ok(ports)
}
