//use std::collections::HashMap;
use std::io::{Read, Write, Error, ErrorKind, Result};
use std::net::{TcpListener, TcpStream};
use std::thread;

// pub struct Message {
//     pub client_domain: String,
//     pub smtp_commands: HashMap<String, String>,
//     pub atm_headers: HashMap<String, String>,
//     pub body: String,
//     pub from: String,
//     pub date: String,
//     pub subject: String,
//     pub to: String,
// }

pub struct Connection {
    pub stream: TcpStream,
    pub id: u32,
    pub buf: Vec<u8>,
}

impl Connection {
    pub fn write_line(&mut self, msg: &str) -> Result<()> {
        let mut msg_csrf = msg.to_owned();
        msg_csrf += "\r\n";
        self.stream.write_all(msg_csrf.as_bytes())?;
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<String> {
        let mut buffer = [0; 1024];
        loop {
            let n = self.stream.read(&mut buffer)?;
            if n == 0 {
                return Err(Error::new(ErrorKind::UnexpectedEof, "Connection closed"));
            }
            self.buf.extend_from_slice(&buffer[..n]);
            if let Some(i) = self.buf.windows(2).position(|window| window == b"\r\n") {
                let line = String::from_utf8(self.buf[..i].to_vec()).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
                self.buf.drain(..=i + 1);
                return Ok(line);
            }
        }
    }

    pub fn handle(&mut self) {
        println!("Handling connection {}", self.id);

        if let Err(e) = self.write_line("220") {
            println!("Failed to send greeting: {}", e);
            return;
        }

        loop {
            match self.read_line() {
                Ok(line) => println!("Received: {}", line),
                Err(e) => {
                    println!("Failed to read line: {}", e);
                    break;
                }
            }
        }
    }
}

pub fn run_server() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:25")?;
    println!("Listening on port 25");

    let mut id = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                id += 1;
                let mut connection = Connection {
                    stream,
                    id,
                    buf: Vec::new(),
                };
                thread::spawn(move || {
                    connection.handle();
                });
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
    Ok(())
}
