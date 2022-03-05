use std::collections::HashMap;
use std::ops::DerefMut;

use crate::room::Room;
use crate::Result;

/// Home has a name and uniquely named rooms
#[derive(Debug)]
pub struct Home {
    pub name: String,
    rooms: HashMap<String, Room>,
}

impl Home {
    /// Returns empty Home
    pub fn new(name: String) -> Self {
        Home {
            name,
            rooms: HashMap::new(),
        }
    }

    /// Returns iterator over pairs (name, room)
    pub fn rooms(&self) -> impl Iterator<Item = (&str, &Room)> {
        self.rooms.iter().map(|(s, r)| (s.as_str(), r))
    }

    /// Returns iterator over room names
    pub fn room_names(&self) -> impl Iterator<Item = &str> {
        self.rooms.keys().map(|s| s.as_str())
    }

    /// Adds room
    /// Can return error in case of failure, f.e. if room name is not unique
    pub fn add_room(&mut self, name: String, room: Room) -> Result<()> {
        let e = self.rooms.entry(name);
        if let std::collections::hash_map::Entry::Vacant(e) = e {
            e.insert(room);
            Ok(())
        } else {
            Err(format!("Room {} already exists", e.key()).into())
        }
    }

    /// Deletes room
    /// Can return error in case of failure, f.e. if there is no such room
    pub fn delete_room(&mut self, name: &str) -> Result<()> {
        self.rooms
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| format!("No such room {}", name).into())
    }

    /// Returns a mutable reference to a room if there is such room
    pub fn room(&mut self, name: &str) -> Option<impl DerefMut<Target = Room> + '_> {
        self.rooms.get_mut(name)
    }

    /// Collects state of all rooms
    pub fn state(&self) -> String {
        self.rooms
            .iter()
            .map(|(name, room)| format!("== {} ==\n{}\n", name, room.state()))
            .fold(String::new(), |acc, s| acc + &s)
    }
}
