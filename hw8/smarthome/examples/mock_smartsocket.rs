use smarthome::{Result, SmartsocketMockServer};
use std::env;
use time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = env::var_os("SMARTSOCKET_ADDR")
        .and_then(|os_string| os_string.into_string().ok())
        .and_then(|string| string.parse().ok())
        .unwrap_or_else(|| "127.0.0.1:55331".to_string());
    let server = SmartsocketMockServer::new(addr, |d: Duration| d.whole_seconds() as u32).await?;

    loop {
        match server.accept().await {
            Ok(mut connection) => {
                let addr = match connection.peer_addr() {
                    Ok(addr) => addr.to_string(),
                    Err(_) => "unknown".into(),
                };

                println!("New client connected: {}", addr);

                tokio::spawn(async move {
                    if connection.handle().await.is_err() {
                        println!("Client disconnected: {}", addr);
                    }
                });
            }
            Err(e) => {
                eprintln!("Can't establish connection: {}", e);
            }
        };
    }
}
