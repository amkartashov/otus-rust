use crate::Result;

#[derive(Default, Debug)]
pub struct SmartSocket {}

impl SmartSocket {
    pub fn new() -> Self {
        Self {}
    }

    pub fn switch(&mut self, _on: bool) -> Result<()> {
        Ok(())
    }

    pub fn power(&self) -> Result<u32> {
        Ok(0)
    }
}
