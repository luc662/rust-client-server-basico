mod usuario;

use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use crate::usuario::Usuario;

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let mut usuario = Usuario { nombre: "Pepe Muleiro".to_owned(), contrasenia: "80880".to_owned() };
            usuario.write_to(&mut stream);
            println!("Sent Hello, awaiting reply...");
            match Usuario::read_from(&mut stream) {
                Ok(a) => {
                    println!("recibi de vuelta a : {}", a.nombre);
                }
                Err(_) => {
                    println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                    stream.shutdown(Shutdown::Both).unwrap();
                }
            }
            {}

            stream.shutdown(Shutdown::Both).unwrap();
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Terminated.");
}