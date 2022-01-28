use crate::room::Room;
use crate::Result;

/// Home has a name and uniquely named rooms
#[derive(Debug)]
pub struct Home {
    pub name: String,
}

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
    pub fn add_room(&mut self, _name: String, _r: Room) -> Result<()> {
        todo!()
    }

    /// Deletes room
    /// Can return error in case of failure, f.e. if there is no such room
    pub fn delete_room(&mut self, _name: &str) -> Result<()> {
        todo!()
    }

    /// Returns a mutable reference to a room if there is such room
    pub fn room(&mut self, _name: &str) -> Option<&mut Room> {
        todo!()
    }

    /// Collects state of all rooms
    pub fn state(&self) -> String {
        todo!()
    }
}
