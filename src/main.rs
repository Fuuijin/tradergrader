use tradergrader::TraderGraderApplication;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = TraderGraderApplication::new();
    app.run().await?;
    Ok(())
}
