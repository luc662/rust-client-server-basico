mod usuario;

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use crate::usuario::Usuario;

fn function(mut text : i32) -> bool {
    text != 1
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match Usuario::read_from(&mut stream) {
        Ok(a) => {
            // echo everything!
//            println!("recived {}", &text);
//            text = "";
            println!("recived {:?}", a);
            if(a.nombre == "Pepe Muleiro".to_owned()) {
                println!("usuario correcto: {}", a.nombre);
            }
            a.write_to(&mut stream).unwrap();

            function(10)
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
