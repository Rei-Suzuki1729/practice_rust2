use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("192.168.11.27:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let msg = b"Hello!VxWorks";
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");
            let mut data = [0 ; 13]; // using 13 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                        let text = from_utf8(&data).unwrap();
                        println!("expected reply: {}", text);
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
