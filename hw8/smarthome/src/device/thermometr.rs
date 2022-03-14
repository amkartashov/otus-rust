use crate::Result;

use std::mem::size_of;

use tokio::io::{Error, ErrorKind};
use tokio::net::{ToSocketAddrs, UdpSocket};

#[derive(Debug)]
pub struct Thermometr {
    socket: UdpSocket,
}

impl Thermometr {
    pub async fn connect<Addrs: ToSocketAddrs>(addrs: Addrs) -> Result<Self> {
        let socket = UdpSocket::bind("[::]:0").await?;
        socket.connect(addrs).await?;
        Ok(Self { socket })
    }

    pub async fn temperature(&self) -> Result<i32> {
        let empty_message = [0; 0];
        let mut buf = [0u8; size_of::<i32>()];
        self.socket.send(&empty_message).await?;
        match self.socket.recv(&mut buf).await {
            Ok(x) if x == size_of::<i32>() => Ok(i32::from_be_bytes(buf)),
            Ok(_) => Err(Error::new(ErrorKind::InvalidData, "less bytes than expected").into()),
            Err(e) => Err(e.into()),
        }
    }
}
