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
    let get = b"GET/HTTP/1.1\r\n";
    let mut buffer = [0; 256];

    if buffer.starts_with(get) {
        stream.read(&mut buffer).unwrap();
        let contents = fs::read_to_string("index.html").unwrap();
        let response = format!("HTTP/1.1 200 ok\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap(); // Nos ayuda a leer la cadena bytes que estamos recibiendo.
        stream.flush().unwrap(); // Esperara e impedira que el programa continue haste que se escriban todo los bytes en la conexion.
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}