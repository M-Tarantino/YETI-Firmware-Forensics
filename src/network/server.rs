use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::util::error::YetiResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum ForensicCommand {
    ScanEntropy,
    MatchSignatures { signatures: Vec<String> },
    Ping,
}

pub async fn start_compute_node(port: u16) -> YetiResult<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    log::info!("YETI Forensic Node listening on: {}", addr);

    loop {
        let (mut socket, peer_addr) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buffer = [0; 2048];
            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let raw_data = &buffer[..n];
                    
                    // BUILDING THE LOGIC:
                    // We attempt to parse the received bytes as a JSON command
                    match serde_json::from_slice::<ForensicCommand>(raw_data) {
                        Ok(command) => {
                            log::info!("Valid command received from {}: {:?}", peer_addr, command);
                            // HERE: Trigger the actual forensic engine based on the command
                            socket.write_all(b"YETI_ACK: COMMAND_PROCESSED").await.ok();
                        },
                        Err(_) => {
                            log::warn!("Received non-command data ({} bytes) from {}. Processing as raw blob...", n, peer_addr);
                            // Fallback: Handle as raw data for entropy analysis
                        }
                    }
                }
                _ => {}
            }
        });
    }
}