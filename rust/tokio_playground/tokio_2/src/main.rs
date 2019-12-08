//use std::io::prelude::*;

fn main() {
    /* synchronous model
    let mut socket = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut buf = [0; 1024];

    socket.write(&mut buf).unwrap();
    socket.read(&mut buf).unwrap();
    */

    use std::net::{SocketAddr, TcpStream};

    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 1)),
        SocketAddr::from(([127, 0, 0, 1], 8081)),
    ];
    if let Ok(stream) = TcpStream::connect(&addrs[..]) {
        println!("Connected to the server: {:?}", stream);
    } else {
        println!("Couldn't connect to server.");
    }
}
