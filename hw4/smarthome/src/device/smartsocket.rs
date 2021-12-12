#[derive(Default, Debug)]
pub struct SmartSocket {}

impl SmartSocket {
    // I want to return Device here so that calling code may use SmartSocket::new() where Device is expected
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> super::Device {
        todo!()
    }

    pub fn switch(&mut self, _on: bool) {
        todo!()
    }

    pub fn power(&self) -> u32 {
        todo!()
    }
}
