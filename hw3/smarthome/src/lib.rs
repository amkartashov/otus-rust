//! # Smarthome crate
//!
//! Usage examples:
//!
//! ```
//! let mut home = Home::new("my home".to_string())
//! home.add_room("kitchen".to_string, Room::default());
//! home.add_room("bedroom".to_string, Room::default());
//! home.room("kitchen").unwrap().add_device("t1".to_string(), )
//! ```

use std::fmt::Debug;

/// Home has a name and uniquely named rooms
#[derive(Debug)]
pub struct Home {}

impl Home {
    /// Returns empty Home
    pub fn new(_name: String) -> Self {
        todo!()
    }

    /// Returns room names
    pub fn rooms(&self) -> Vec<&str> {
        todo!()
    }

    /// Adds room
    /// Can return error in case of failure, f.e. if room name is not unique
    pub fn add_room(&mut self, _name: String, _r: Room) -> Result<(), String> {
        todo!()
    }

    /// Deletes room
    /// Can return error in case of failure, f.e. if there is no such room
    pub fn delete_room(&mut self, _name: &str) -> Result<(), String> {
        todo!()
    }

    /// Returns a mutable reference to a room
    pub fn room(&mut self, _name: &str) -> Result<&mut Room, String> {
        todo!()
    }

    /// Collects state of all rooms
    pub fn state(&self) -> String {
        todo!()
    }
}

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
    pub fn device(&mut self, _name: &str) -> Result<&mut Device, String> {
        todo!()
    }

    /// Collects state of all devices
    pub fn state(&self) -> String {
        todo!()
    }
}

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

#[derive(Default, Debug)]
pub struct Thermometr {}

impl Thermometr {
    pub fn temperature(&self) -> i32 {
        todo!()
    }
}

#[derive(Default, Debug)]
pub struct SmartSocket {}

impl SmartSocket {
    pub fn switch(&mut self, _on: bool) {
        todo!()
    }

    pub fn power(&self) -> u32 {
        todo!()
    }
}
