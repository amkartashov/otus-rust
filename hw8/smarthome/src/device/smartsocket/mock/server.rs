use crate::Result;
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};

use super::super::protocol::{recv_command, send_bool, send_u32, Command::*};
use super::smartsocket::Smartsocket;

/// Represents Smartsocket server, that can accept incoming connections.
pub struct Server {
    tcp: TcpListener,
    socket: Smartsocket,
}

impl Server {
    /// Binds server to specified address
    /// Initializes internal state and takes power load function
    pub fn new<Addrs: ToSocketAddrs, F: 'static + Send + Fn(time::Duration) -> u32>(
        addrs: Addrs,
        load_func: F,
    ) -> Result<Self> {
        let tcp = TcpListener::bind(addrs)?;
        let socket = Smartsocket::new(load_func);
        Ok(Self { tcp, socket })
    }

    /// Blocking iterator for incoming connections.
    pub fn incoming(&self) -> impl Iterator<Item = Result<Connection>> + '_ {
        // map stream of Result<TcpStream, std::io::Error> to
        // stream of Result<Connection, crate::Error>
        self.tcp.incoming().map(|stream| {
            stream
                .map(|stream| Connection {
                    stream,
                    socket: self.socket.clone(),
                })
                .map_err(|e| e.into())
        })
    }
}

/// Represents connection from client.
pub struct Connection {
    stream: TcpStream,
    socket: Smartsocket,
}

impl Connection {
    /// Send response to client
    pub fn handle(&mut self) -> Result<()> {
        loop {
            let command = recv_command(&mut self.stream)?;
            match command {
                SwitchOn => self.socket.switch(true),
                SwitchOff => self.socket.switch(false),
                GetState => {
                    let (is_on, power) = self.socket.state();
                    send_bool(is_on, &mut self.stream)?;
                    send_u32(power, &mut self.stream)?;
                }
            };
        }
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
