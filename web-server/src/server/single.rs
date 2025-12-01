use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub fn listen(host: &str, port: u16) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf)?;

    // let mut file = File::open("hello.html").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    let contents = include_str!("../../hello.html");

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    let _ = stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
