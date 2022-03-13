use std::env;
use std::io::Result;
use std::net::UdpSocket;

/// will send back 25i32 to any client which sends any message (even empty)
fn main() -> Result<()> {
    let addr = env::var_os("THERMOMETR_ADDR")
        .and_then(|os_string| os_string.into_string().ok())
        .and_then(|string| string.parse().ok())
        .unwrap_or_else(|| "127.0.0.1:55332".to_string());

    let socket = UdpSocket::bind(addr)?;

    let mut empty_message_receiver = [0; 0];

    loop {
        let (_, src) = socket.recv_from(&mut empty_message_receiver)?;
        println!("New client connected: {}", src);
        socket.send_to(&(25i32.to_be_bytes()), &src)?;
    }
}
