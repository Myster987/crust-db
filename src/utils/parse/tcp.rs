use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub enum ParseError {
    CoudNotReadStream,
    CoudNotWriteStream,
}

type ParseResult<T> = Result<T, ParseError>;

pub async fn parse_stream_to_string(stream: &mut TcpStream) -> ParseResult<String> {
    let mut parsed_string = String::new();
    match stream.read_to_string(&mut parsed_string).await {
        Ok(_) => Ok(parsed_string.trim().to_owned()),
        Err(e) => {
            log::warn!("Coudn't read tcp stream: {}", e);
            Err(ParseError::CoudNotReadStream)
        }
    }
}

pub async fn parse_string_to_stream(stream: &mut TcpStream, payload: String) -> ParseResult<()> {
    match stream.write_all(payload.as_bytes()).await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::warn!("{}", e);

            Err(ParseError::CoudNotWriteStream)
        }
    }
}
