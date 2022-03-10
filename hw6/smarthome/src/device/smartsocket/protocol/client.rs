use super::{recv_bool, recv_u32, send_command, Command::*};
use crate::Result;
use std::net::{TcpStream, ToSocketAddrs};

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}

impl Client {
    /// Try to connect to specified address and perform handshake.
    pub fn connect<Addrs: ToSocketAddrs>(addrs: Addrs) -> Result<Self> {
        let stream = TcpStream::connect(addrs)?;
        Ok(Self { stream })
    }

    pub fn switch(&mut self, on: bool) -> Result<()> {
        match on {
            true => send_command(SwitchOn, &self.stream),
            false => send_command(SwitchOff, &self.stream),
        }
    }

    /// Returns pair (on: bool, power: u32)
    pub fn state(&self) -> Result<(bool, u32)> {
        send_command(GetState, &self.stream)?;
        Ok((recv_bool(&self.stream)?, recv_u32(&self.stream)?))
    }
}
