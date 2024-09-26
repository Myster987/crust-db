use tokio::net::{TcpListener, TcpStream};
use utils::connection::Connection;

use std::env;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let host_env_name = "CRUST_HOST";

    let host = match env::var(host_env_name) {
        Ok(val) => val,
        Err(_) => "127.0.0.1:3902".to_owned(),
    };

    let listner = TcpListener::bind(&host).await?;

    log::info!("Crust-db listening to tcp on: {}", listner.local_addr()?);

    loop {
        let (socket, _) = listner.accept().await?;
        tokio::spawn(async move {
            process(socket).await;
        });
    }

    Ok(())
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Some(payload) = connection.parse_incoming_payload().await {
        println!("{}", payload);
    }

    // if let Some(content) = connection.parse_incoming_payload().await {
    //     log::info!("{}", content.payload());
    // }

    // let parsed_data = tcp::Parse::new(&mut socket).await;
    // if let Ok(text) = parsed_data {

    // }
}
