use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::util::error::{YetiResult, YetiError};

/// Sends a forensic task (e.g., a buffer for entropy analysis) to a remote node.
pub async fn send_forensic_task(target: &str, payload: &[u8]) -> YetiResult<()> {
    log::info!("Connecting to remote compute node at {}...", target);
    
    let mut stream = TcpStream::connect(target).await
        .map_err(|e| YetiError::Network(format!("Failed to connect: {}", e)))?;

    log::debug!("Sending payload of {} bytes...", payload.len());
    
    // Send the raw data
    stream.write_all(payload).await?;
    
    // Shutdown the write half so the server knows we are done sending
    stream.shutdown().await?;

    // Wait for the server's response (ACK)
    let mut response = [0; 8];
    let n = stream.read(&mut response).await?;
    
    if &response[..n] == b"YETI_ACK" {
        log::info!("Task accepted by remote node: {}", target);
        Ok(())
    } else {
        Err(YetiError::Network("Remote node returned invalid response".into()))
    }
}