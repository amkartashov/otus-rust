use std::io::{Error, ErrorKind};
use std::mem::size_of;
use std::net::{ToSocketAddrs, UdpSocket};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;

use crate::Result;

#[derive(Debug)]
pub struct Thermometr {
    temperature: Arc<Mutex<Result<i32>>>,
    /// flag that struct is dropped
    /// signals monitoring thread to exit
    dropped: Arc<AtomicBool>,
}

impl Thermometr {
    /// Starts long-liliving thread to update temperature every second
    /// which exits if Thermometr::drop() is called
    pub fn connect<Addrs: ToSocketAddrs>(addrs: Addrs) -> Self {
        let res = Self {
            temperature: Arc::new(Mutex::new(Err(Error::new(
                ErrorKind::NotConnected,
                "not connected yet",
            )
            .into()))),
            dropped: Arc::new(AtomicBool::default()),
        };

        let temperature = Arc::clone(&res.temperature);
        let dropped = Arc::clone(&res.dropped);

        // connect first
        match UdpSocket::bind("[::]:0") {
            Err(e) => *temperature.lock().unwrap() = Err(e.into()),
            Ok(socket) => {
                if let Err(e) = socket.connect(addrs) {
                    *temperature.lock().unwrap() = Err(e.into());
                } else {
                    thread::spawn(move || {
                        // keep current temperature to avoid frequent write locks
                        let mut current_t = None;
                        let empty_message = [0; 0];
                        let mut buf = [0u8; size_of::<i32>()];
                        loop {
                            if let Err(e) = socket.send(&empty_message) {
                                *temperature.lock().unwrap() = Err(e.into());
                                current_t = None;
                            } else {
                                match socket.recv(&mut buf) {
                                    Ok(x) if x == size_of::<i32>() => {
                                        let new_t = i32::from_be_bytes(buf);
                                        // update current_t if needed
                                        if current_t != Some(new_t) {
                                            current_t = Some(new_t);
                                            *temperature.lock().unwrap() = Ok(new_t);
                                        }
                                    }
                                    Ok(_) => {
                                        current_t = None;
                                        *temperature.lock().unwrap() = Err(Error::new(
                                            ErrorKind::InvalidData,
                                            "less bytes than expected",
                                        )
                                        .into());
                                    }
                                    Err(e) => {
                                        current_t = None;
                                        *temperature.lock().unwrap() = Err(e.into());
                                    }
                                };
                            };

                            // check if Self is dropped
                            if dropped.load(Ordering::Relaxed) {
                                break;
                            };

                            thread::sleep(std::time::Duration::from_secs(1));
                        }
                    });
                }
            }
        };

        res
    }

    pub fn temperature(&self) -> Result<i32> {
        self.temperature.lock().unwrap().clone()
    }
}

impl Drop for Thermometr {
    fn drop(&mut self) {
        self.dropped.store(false, Ordering::Relaxed);
    }
}
