use crate::Result;
use std::io;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

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
    pub async fn new<Addrs: ToSocketAddrs, F: 'static + Send + Fn(time::Duration) -> u32>(
        addrs: Addrs,
        load_func: F,
    ) -> Result<Self> {
        let tcp = TcpListener::bind(addrs).await?;
        let socket = Smartsocket::new(load_func);
        Ok(Self { tcp, socket })
    }

    /// Blocking iterator for incoming connections.
    pub async fn accept(&self) -> Result<Connection> {
        // map stream of Result<TcpStream, std::io::Error> to
        // stream of Result<Connection, crate::Error>
        self.tcp
            .accept()
            .await
            .map(|(stream, _)| Connection {
                stream,
                socket: self.socket.clone(),
            })
            .map_err(|e| e.into())
    }
}

/// Represents connection from client.
pub struct Connection {
    stream: TcpStream,
    socket: Smartsocket,
}

impl Connection {
    /// Send response to client
    pub async fn handle(&mut self) -> Result<()> {
        loop {
            let command = recv_command(&mut self.stream).await?;
            match command {
                SwitchOn => self.socket.switch(true),
                SwitchOff => self.socket.switch(false),
                GetState => {
                    let (is_on, power) = self.socket.state();
                    send_bool(is_on, &mut self.stream).await?;
                    send_u32(power, &mut self.stream).await?;
                }
            };
        }
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> io::Result<std::net::SocketAddr> {
        self.stream.peer_addr()
    }
}
