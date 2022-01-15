use crate::device::Device;

/// Room with uniquely named devices
#[derive(Default, Debug)]
pub struct Room {}

impl Room {
    /// Returns list of device names
    pub fn devices(&self) -> Vec<&str> {
        todo!()
    }

    /// Adds device
    pub fn add_device(&mut self, _name: String, _descr: String, _d: Device) -> Result<(), String> {
        todo!()
    }

    /// Deletes device
    pub fn delete_device(&mut self, _name: &str) -> Result<(), String> {
        todo!()
    }

    /// Returns a reference to a device
    pub fn device(&mut self, _name: &str) -> Option<&mut Device> {
        todo!()
    }

    /// Collects state of all devices
    pub fn state(&self) -> String {
        todo!()
    }
}
