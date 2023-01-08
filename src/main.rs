// Uncomment this block to pass the first stage
use std::{net::{TcpListener, TcpStream}, io::{self, BufRead, Write}};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                loop {
                    pingpong(&mut _stream);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn pingpong(mut stream: &TcpStream) {
    let mut reader = io::BufReader::new(&mut stream);
    let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    reader.consume(received.len());
    println!("{:?}", String::from_utf8(received).unwrap());

    println!("accepted new connection");

    let response = "+PONG\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
