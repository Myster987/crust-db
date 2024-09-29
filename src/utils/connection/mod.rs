use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

pub struct Connection {
    stream: Stream,
}

pub struct Stream {
    writer: BufWriter<TcpStream>,
    buffer: Vec<u8>,
}

pub enum ParseError {
    CoudNotReadStream,
    CoudNotWriteStream,
}

// -- Stream -- Constructor
impl Stream {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            writer: BufWriter::new(stream),
            buffer: Vec::with_capacity(8 * 1024),
        }
    }
}

impl Stream {
    pub async fn parse_stream_to_string(&mut self) -> Result<Option<String>, ParseError> {
        let bytes_read = self.writer.read_buf(&mut self.buffer).await;

        match bytes_read {
            Ok(b) => {
                if b == 0 {
                    return Ok(None);
                }
                let payload = match String::from_utf8(self.buffer.clone()) {
                    Ok(p) => p.trim().to_owned(),
                    Err(_) => return Err(ParseError::CoudNotReadStream),
                };
                self.buffer.clear();
                Ok(Some(payload))
            }
            Err(e) => {
                log::warn!("Coudn't read tcp stream: {}", e);
                Err(ParseError::CoudNotReadStream)
            }
        }
    }

    pub async fn parse_string_to_stream(&mut self, payload: String) -> Result<(), ParseError> {
        match self.buffer.write_all(payload.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(e) => {
                log::warn!("{}", e);
                Err(ParseError::CoudNotWriteStream)
            }
        }
    }
}

// -- Connection -- Constructor
impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: Stream::new(stream),
        }
    }
}

// -- Connection -- Methods
impl Connection {
    pub async fn parse_incoming_payload(&mut self) -> Option<String> {
        let parsed_payload = self.stream.parse_stream_to_string().await;
        match parsed_payload {
            Ok(payload) => payload,
            Err(_) => None,
        }
    }

    pub async fn send_payload(&mut self, payload: String) -> Result<(), ParseError> {
        Ok(self.stream.parse_string_to_stream(payload).await?)
    }
}
