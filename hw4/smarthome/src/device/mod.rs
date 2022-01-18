mod smartsocket;
mod thermometr;

pub use smartsocket::SmartSocket;
pub use thermometr::Thermometr;

#[derive(Debug)]
#[non_exhaustive]
pub enum Device {
    Thermometr(Thermometr),
    SmartSocket(SmartSocket),
}

#[derive(Debug, Clone)]
pub struct DeviceError {}

impl Device {
    pub fn state(&self) -> Result<String, DeviceError> {
        todo!()
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
