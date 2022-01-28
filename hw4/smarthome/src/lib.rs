//! # Smarthome crate
//!
//! Usage examples:
//!
//! ```
//! let mut home = Home::new("my home".to_string());
//! home.add_room("kitchen".to_string, Room::default());
//! home.add_room("bedroom".to_string, Room::default());
//! home.room("kitchen").unwrap().add_device("t1".to_string(), )
//! ```

mod device;
mod error;
mod home;
mod room;

pub use device::{Device, SmartSocket, Thermometr};
pub use error::{Error, Result};
pub use home::Home;
pub use room::Room;

#[cfg(test)]
mod tests;
