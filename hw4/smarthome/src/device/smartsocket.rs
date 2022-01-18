#[derive(Default, Debug)]
pub struct SmartSocket {}

impl SmartSocket {
    pub fn new() -> Self {
        todo!()
    }

    pub fn switch(&mut self, _on: bool) -> Result<(), super::DeviceError> {
        todo!()
    }

    pub fn power(&self) -> Result<u32, super::DeviceError> {
        todo!()
    }
}
