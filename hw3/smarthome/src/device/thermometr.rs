#[derive(Default, Debug)]
pub struct Thermometr {}

impl Thermometr {
    // I want to return Device here so that calling code may use Thermometr::new() where Device is expected
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> super::Device {
        todo!()
    }

    pub fn temperature(&self) -> i32 {
        todo!()
    }
}
