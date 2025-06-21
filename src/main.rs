use tradergrader::StandaloneMcpServer;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--health" {
        let server = StandaloneMcpServer::new();
        server.health_check().await?;
        return Ok(());
    }
    
    let server = StandaloneMcpServer::new();
    server.run().await?;
    Ok(())
}
