use std::net::TcpStream;
use std::io::{Read, Write, Result};
use std::thread;
use std::time::Duration;
use smtp_server::server;

fn start_server() {
    thread::spawn(|| {
        server::run_server().unwrap();
    });

    // Give the server a moment to start up
    thread::sleep(Duration::from_secs(1));
}

fn send_message(msg: &str) -> Result<String> {
    let mut stream = TcpStream::connect("127.0.0.1:25")?;
    stream.write_all(msg.as_bytes())?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
}

#[test]
fn test_server_greeting() {
    start_server();
    match send_message("EHLO localhost\r\n") {
        Ok(response) => {
            assert!(response.starts_with("220"));
            println!("Response: {}", response);
        },
        Err(e) => {
            panic!("Failed to send message {}", e);
        },
    }
}
