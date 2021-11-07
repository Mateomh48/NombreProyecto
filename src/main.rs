use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7373").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        }
    }

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 256];
    stream.read(&mut buffer).unwrap();
    let contents = fs::read_to_string("index.html").unwrap();
    let response = format!("HTTP/1.1 200 ok\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap(); // Nos ayuda a leer la cadena bytes que estamos recibiendo.
    stream.flush().unwrap(); // Esperara e impedira que el programa continue haste que se escriban todo los bytes en la conexion.
}