use std::collections::HashMap;
use std::io::{Read, Write, Error, ErrorKind, Result};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fmt::Arguments;

pub struct Message {
    pub client_domain: String,
    pub smtp_commands: HashMap<String, String>,
    pub atm_headers: HashMap<String, String>,
    pub body: String,
    pub from: String,
    pub date: String,
    pub subject: String,
    pub to: String,
}

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

    fn log_info(&self, msg: &str, args: Option<Arguments>) {
        let peer_address = self.stream.peer_addr().expect("Could not get the remote address");
        if let Some(arguments) = args {
            println!("[INFO] [{}:{}] {}", self.id, peer_address, format_args!("{} {}", msg, arguments));
        } else {
            println!("[INFO] [{}:{}] {}", self.id, peer_address, msg);
        }
    }

    fn log_error(&self, e:Error) {
        let peer_address = self.stream.peer_addr().expect("Could not get the remote address");
        println!("[ERROR] [{}:{}] {}", self.id, peer_address, e);
    }

    pub fn read_multiline(&mut self) -> Result<String> {
        loop{
            let mut no_more_reads = false;
            for i in 0..self.buf.len(){
                if i > 1 &&
                self.buf[i] != b' ' &&
                self.buf[i] != b'\t' &&
                self.buf[i-2] != b'\r' &&
                self.buf[i-1] != b'\n'{
                    let line = String::from_utf8_lossy(&self.buf[..i-2]).into_owned();
                    self.buf = self.buf[i..].to_vec();
                    return Ok(line);
                }
                no_more_reads = self.is_body_close(i);
            }

            if !no_more_reads{
                let mut b = vec![0;1024];
                let n = self.stream.read(&mut b)?;
                self.buf.extend_from_slice(&b[..n]);
            }
            
        }
    }

    fn is_body_close(&self, i:usize) -> bool{
        return i > 4 && 
        self.buf[i-4] == b'\r' &&
        self.buf[i-3] == b'\n' &&
        self.buf[i-2] == b'.' &&
        self.buf[i-1] == b'\r' &&
        self.buf[i-0] == b'\n'
    }

    fn read_to_end_of_body(&mut self) -> Result<String>{
        loop{
            for i in 0..self.buf.len(){
                if self.is_body_close(i){
                    let line: String = String::from_utf8_lossy(&self.buf[..i-4]).into_owned();
                    return Ok(line);
                }
            }

            let mut b = [0;1024];
            let n = self.stream.read(&mut b)?;
            self.buf.extend_from_slice(&b[..n]);
        }   
    }

    pub fn handle(&mut self) {
        println!("Handling connection {}", self.id);

        if let Err(e) = self.write_line("220") {
            self.log_error(e);
        }

        self.log_info("Awaiting EHLO", None);
        let mut line = String::new();

        match self.read_line(){
            Ok(l)=>{
                if !l.starts_with("EHLO"){
                    self.log_error(Error::new(ErrorKind::Other, format!("Expected EHLO Got: {}",l)));
                    return 
                }
                line = l
            },
            Err(e)=>{
                self.log_error(e);
            }
        }

        let mut msg = Message{
            client_domain: line[5..].to_string(),
            smtp_commands:HashMap::new(),
            atm_headers:HashMap::new(),
            body: String::new(),
            from:String::new(),
            date:String::new(),
            subject:String::new(),
            to:String::new(),
        };

        if let Err(e) = self.write_line("250"){
            self.log_error(e);
        }

        self.log_info("Done EHLO", None);

        loop{
            let mut line = String::new();
            match self.read_line(){
                Ok(l) =>{                    
                    if l.trim().is_empty(){
                        self.log_error(Error::new(ErrorKind::Other, format!("Line is empty {}",l))); 
                        break;
                    }
                    //parts = line.splitn(2,":").collect();                    
                    line = l;
                },
                Err(e)=>{
                    self.log_error(e);
                }

            }
            let parts:Vec<&str> = line.splitn(2, ":").collect();

            if parts.len() != 2{
                continue;
            }

            let command = parts[0].to_uppercase();
            let value = parts[1].trim().to_string();

            if command == "DATA"{
                if let Err(e) = self.write_line("354"){
                    self.log_error(e);
                }
                 
                break;
            }
            msg.smtp_commands.insert(command,value);
        }

        self.log_info("Done SMTP headers, reading ARPA text message headers",None);

        loop {
           
            let mut line = String::new();
            match self.read_multiline(){
                Ok(l) => {
                    if l.trim().is_empty(){
                        self.log_error(Error::new(ErrorKind::Other, format!("lLine is empty at reading ARPA {}",l)));
                        break;
                    }
                    line = l;
                },
                Err(e)=>{
                    self.log_error(e);
                },
            }        
            let pieces:Vec<&str> = line.splitn(2, ":").collect();

            let atm_headers = pieces[0].to_uppercase();
            let atm_value = pieces[1].trim().to_string(); // so it owns and not just the reference
            msg.atm_headers.insert(atm_headers.clone(),atm_value.clone());

            if atm_headers == "SUBJECT" {
                msg.subject = atm_value;
            } else if atm_headers == "TO"{
                msg.to = atm_value;
            } else if atm_headers == "FROM"{
                msg.from = atm_value;
            }else if atm_headers == "DATE"{
                msg.date = atm_value;
            }
        }
        self.log_info("Done ARPA text message headers, reading body", None);

        match self.read_to_end_of_body(){
            Ok(body)=> {
                let _ = self.write_line("250 OK");
                self.log_info(&format!("Message: {} \n", body),None);
            },
            Err(e)=>{
                self.log_error(e);
            }
        }

    }
}

pub fn run_server() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:2525")?;
    println!("Listening on port 2525");

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
