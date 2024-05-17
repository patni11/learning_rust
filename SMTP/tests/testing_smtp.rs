use std::net::TcpStream;
use std::io::{Read,Write,Result};
use std::thread;
use std::time::Duration;
use smtp_server::server;

fn start_server(){
    thread::spawn(||{
        server::run_server().unwrap();
    });
    thread::sleep(Duration::from_secs(1));
}

fn send_message(msg: &str, stream: &mut TcpStream) -> Result<String> {
    
    stream.write_all(msg.as_bytes())?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
}

#[test]
fn test_smtp_server(){
    start_server();
    let mut stream = TcpStream::connect("127.0.0.1:2525").expect("Could not connect to server");

    // //check connection
    match send_message("",&mut stream) {
        Ok(response) => {
            assert!(response.starts_with("220"));
            println!("Response: {}", response);
        },
        Err(e) => {
            panic!("Failed to send message {}", e);
        },
    }

//    check EHLO
    match send_message("EHLO example.com\r\n", &mut stream) {
        Ok(response) => {
            assert!(response.starts_with("250"));
            println!("Response: {}", response);
        },
        Err(e) => {
            panic!("Failed to send message {}", e);
        },
    }
    // println!("DONE WITH EHLO TEST");
    // send_message("FROM:<sender@example.com>\r\n", &mut stream).expect("Couldn't send MAIL");
    // println!("SENT MAIL");
    // send_message("TO:<recipient@example.com>\r\n", &mut stream).expect("Couldn't send RCPT");
    // println!("SENT RCPT");
//    check for Data
    match send_message("MAIL FROM:<sender@example.com>\r\nRCPT TO:<recipient@example.com>\r\nDATA\r\n", &mut stream) {
        Ok(response) => {
            assert!(response.starts_with("354"));
            println!("Response: {}", response);
        },
        Err(e) => {
            panic!("Failed to send message {}", e);
        },
    }
}


