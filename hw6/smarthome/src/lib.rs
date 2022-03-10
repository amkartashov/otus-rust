//! # Smarthome crate
//!
//! Usage examples:
//!
//! ```
//! use smarthome::{Home, Room, Thermometr};
//!
//! let mut home = Home::new("my home".into());
//! home.add_room("kitchen".into(), Room::default());
//! home.add_room("bedroom".into(), Room::default());
//! home.room("kitchen").unwrap()
//!     .add_device("t1".into(), "thermometr".into(), Thermometr::new().into());
//! ```

mod device;
mod error;
mod home;
mod room;

pub use device::{Device, SmartSocket, SmartsocketMockServer, Thermometr};
pub use error::{Error, Result};
pub use home::Home;
pub use room::Room;

#[cfg(test)]
mod tests;
