mod smartsocket;
mod thermometr;

pub use smartsocket::SmartSocket;
pub use thermometr::Thermometr;

#[derive(Debug)]
pub enum Device {
    Thermometr(Thermometr),
    SmartSocket(SmartSocket),
}

impl Device {
    pub fn state(&self) -> String {
        todo!()
    }
}
