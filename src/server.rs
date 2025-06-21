//! Standalone MCP server implementation

use crate::mcp::McpHandler;
use serde_json::Value;
use std::io::{self, BufRead, Write, BufReader, BufWriter};
use tokio::time::{timeout, Duration};

/// Standalone MCP server that can handle persistent connections
pub struct StandaloneMcpServer {
    handler: McpHandler,
}

impl StandaloneMcpServer {
    pub fn new() -> Self {
        Self {
            handler: McpHandler::new("TraderGrader".to_string(), "0.1.0".to_string()),
        }
    }

    /// Run the server with proper connection handling
    pub async fn run(&self) -> anyhow::Result<()> {
        eprintln!("TraderGrader MCP Server starting on stdio...");
        
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut reader = BufReader::new(stdin.lock());
        let mut writer = BufWriter::new(stdout.lock());
        
        let mut line = String::new();
        
        loop {
            line.clear();
            
            // Read with timeout to handle client disconnections
            match timeout(Duration::from_secs(1), async {
                reader.read_line(&mut line)
            }).await {
                Ok(Ok(0)) => {
                    // EOF - client disconnected
                    eprintln!("Client disconnected");
                    break;
                }
                Ok(Ok(_)) => {
                    if line.trim().is_empty() {
                        continue;
                    }
                    
                    // Process the message
                    match serde_json::from_str::<Value>(&line) {
                        Ok(message) => {
                            let response = self.handler.handle_message(message).await;
                            
                            // Only send response if it's not null (notifications return null)
                            if !response.is_null() {
                                if let Ok(response_str) = serde_json::to_string(&response) {
                                    if writeln!(writer, "{}", response_str).is_err() {
                                        eprintln!("Failed to write response");
                                        break;
                                    }
                                    if writer.flush().is_err() {
                                        eprintln!("Failed to flush response");
                                        break;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse message: {}", e);
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("IO error: {}", e);
                    break;
                }
                Err(_) => {
                    // Timeout - continue loop to check for new input
                    continue;
                }
            }
        }
        
        eprintln!("MCP Server shutting down");
        Ok(())
    }

    /// Run a simple health check
    pub async fn health_check(&self) -> anyhow::Result<()> {
        println!("TraderGrader MCP Server is healthy");
        Ok(())
    }
}

impl Default for StandaloneMcpServer {
    fn default() -> Self {
        Self::new()
    }
}