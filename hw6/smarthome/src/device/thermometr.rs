use crate::Result;

#[derive(Default, Debug)]
pub struct Thermometr {}

impl Thermometr {
    pub fn new() -> Self {
        Self {}
    }

    pub fn temperature(&self) -> Result<i32> {
        Ok(0)
    }
}
