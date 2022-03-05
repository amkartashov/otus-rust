mod smartsocket;
mod thermometr;

use crate::Result;
pub use smartsocket::SmartSocket;
pub use thermometr::Thermometr;

#[derive(Debug)]
#[non_exhaustive]
pub enum Device {
    Thermometr(Thermometr),
    SmartSocket(SmartSocket),
}

impl Device {
    pub fn state(&self) -> Result<String> {
        match &self {
            Device::Thermometr(_t) => Ok("".into()),
            Device::SmartSocket(_s) => Ok("".into()),
        }
    }
}

impl From<Thermometr> for Device {
    fn from(t: Thermometr) -> Self {
        Device::Thermometr(t)
    }
}

impl From<SmartSocket> for Device {
    fn from(s: SmartSocket) -> Self {
        Device::SmartSocket(s)
    }
}
