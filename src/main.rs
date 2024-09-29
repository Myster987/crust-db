mod utils;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let database_server = server::DatabaseServer::new();
    database_server.start().await?;

    Ok(())
}
