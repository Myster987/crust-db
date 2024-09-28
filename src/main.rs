mod utils;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_server = server::DatabaseServer::new();
    database_server.start().await?;

    Ok(())
}
