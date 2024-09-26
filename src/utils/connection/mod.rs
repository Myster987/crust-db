use tokio::net::TcpStream;

use super::parse;

pub struct Connection {
    stream: TcpStream,
}

// -- Connection -- Constructor
impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
}

// -- Connection -- Methods
impl Connection {
    pub async fn parse_incoming_payload(&mut self) -> Option<String> {
        let parsed_payload = parse::tcp::parse_stream_to_string(&mut self.stream).await;
        match parsed_payload {
            Ok(payload) => Some(payload),
            Err(_) => None
        }
    }

    pub async fn send_payload(&mut self, payload: String) -> Result<(), parse::tcp::ParseError> {
        let send_result = parse::tcp::parse_string_to_stream(&mut self.stream, payload).await?;
        Ok(send_result)
    }
}
