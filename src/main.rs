use std::net::{TcpListener, TcpStream};


fn main() {
    let litstener = TcpListener::bind("127.0.0.1:7373").unwrap();
    for stream in litstener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client!");
            }
            Err(e) => {
                println!("Failed connection")
            }
        }
    }
    println!("Hello, world!");
}
