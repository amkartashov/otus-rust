use super::{recv_bool, recv_u32, send_command, Command::*};
use crate::Result;

use std::ops::DerefMut;
use std::sync::Mutex;
use tokio::net::{TcpStream, ToSocketAddrs};

#[derive(Debug)]
pub struct Client {
    // stream should be protected with mutex
    // to prevent interwining of send/recv
    stream: Mutex<TcpStream>,
}

impl Client {
    /// Try to connect to specified address and perform handshake.
    pub async fn connect<Addrs: ToSocketAddrs>(addrs: Addrs) -> Result<Self> {
        let stream = TcpStream::connect(addrs).await?;
        Ok(Self {
            stream: Mutex::new(stream),
        })
    }

    pub async fn switch(&mut self, on: bool) -> Result<()> {
        let mut stream = self.stream.lock().unwrap();
        match on {
            true => send_command(SwitchOn, stream.deref_mut()).await,
            false => send_command(SwitchOff, stream.deref_mut()).await,
        }
    }

    /// Returns pair (on: bool, power: u32)
    pub async fn state(&self) -> Result<(bool, u32)> {
        let mut stream = self.stream.lock().unwrap();
        send_command(GetState, stream.deref_mut()).await?;
        Ok((
            recv_bool(stream.deref_mut()).await?,
            recv_u32(stream.deref_mut()).await?,
        ))
    }
}
