use std::env;

use db::command::Command;
use tokio::net::{TcpListener, TcpStream};

use crate::utils::connection::Connection;

pub mod db;

#[allow(non_snake_case)]
pub struct DatabaseServer {
    CRUST_HOST: String,
}

// -- DatabaseServer -- Constructor
impl DatabaseServer {
    pub fn new() -> Self {
        let host_env_name = "CRUST_HOST";

        let host = match env::var(host_env_name) {
            Ok(val) => val,
            Err(_) => "127.0.0.1:3902".to_owned(),
        };

        Self { CRUST_HOST: host }
    }
}

// -- DatabaseServer -- Methods
impl DatabaseServer {
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listner = TcpListener::bind(&self.CRUST_HOST).await.unwrap();

        log::info!("Crust-db listening to tcp on: {}", listner.local_addr()?);

        loop {
            let (socket, _) = listner.accept().await?;
            tokio::spawn(async move {
                DatabaseServer::process(socket).await;
            });
        }
    }

    async fn process(socket: TcpStream) {
        let mut connection = Connection::new(socket);

        while let Some(payload) = connection.parse_incoming_payload().await {
            let command = Command::new(payload);
            
            match command {
                Ok(r) => { log::info!("{:?}", r)},
                Err(e) => { log::warn!("{:?}", e)}
            }
        }
    }
}
