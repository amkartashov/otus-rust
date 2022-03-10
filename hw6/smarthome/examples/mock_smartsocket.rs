use smarthome::{Result, SmartsocketMockServer};
use std::env;
use std::thread;
use time::Duration;

fn main() -> Result<()> {
    let addr = env::var_os("SMARTSOCKET_ADDR")
        .and_then(|os_string| os_string.into_string().ok())
        .and_then(|string| string.parse().ok())
        .unwrap_or_else(|| "127.0.0.1:55331".to_string());
    let server = SmartsocketMockServer::new(addr, |d: Duration| d.whole_seconds() as u32)?;

    for connection in server.incoming() {
        let mut connection = match connection {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Can't establish connection: {}", e);
                continue;
            }
        };

        let addr = match connection.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        println!("New client connected: {}", addr);

        thread::spawn(move || {
            if connection.handle().is_err() {
                println!("Client disconnected: {}", addr);
            }
        });
    }

    Ok(())
}
