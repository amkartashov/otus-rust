use std::collections::HashMap;
use std::ops::DerefMut;

use crate::device::Device;
use crate::Result;

/// Room with uniquely named devices
#[derive(Default, Debug)]

pub struct Room {
    devices: HashMap<String, (String, Device)>,
}

impl Room {
    /// Returns empty Room
    pub fn new() -> Self {
        Room {
            devices: HashMap::new(),
        }
    }

    /// Returns iterator over tuples (name, description, device)
    pub fn devices(&self) -> impl Iterator<Item = (&str, &str, &Device)> {
        self.devices
            .iter()
            .map(|(name, (descr, device))| (name.as_str(), descr.as_str(), device))
    }

    /// Returns iterator over room names
    pub fn device_names(&self) -> impl Iterator<Item = &str> {
        self.devices.keys().map(|s| s.as_str())
    }

    /// Adds device
    pub fn add_device(&mut self, name: String, descr: String, device: Device) -> Result<()> {
        for (_, d) in self.devices.values() {
            if std::mem::discriminant(d) == std::mem::discriminant(&device) {
                return Err("Room already has device of the type".into());
            }
        }

        let e = self.devices.entry(name);
        if let std::collections::hash_map::Entry::Vacant(e) = e {
            e.insert((descr, device));
            Ok(())
        } else {
            Err(format!("Device {} already exists", e.key()).into())
        }
    }

    /// Deletes device
    pub fn delete_device(&mut self, name: &str) -> Result<()> {
        self.devices
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| format!("No such device {}", name).into())
    }

    /// Returns a reference to a device
    pub fn device(&mut self, name: &str) -> Option<impl DerefMut<Target = Device> + '_> {
        self.devices.get_mut(name).map(|(_descr, device)| device)
    }

    /// Collects state of all devices
    pub fn state(&self) -> String {
        self.devices
            .iter()
            .map(|(name, (_descr, device))| {
                format!(
                    "{}: {}",
                    name,
                    device.state().unwrap_or_else(|e| format!("error: {}", e))
                )
            })
            .fold(String::new(), |acc, s| acc + &s + "\n")
    }
}
