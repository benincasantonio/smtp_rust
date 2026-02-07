use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead};

struct Client {
    ip: String,
    port: u16
}

fn listen_client() -> std::io::Result<TcpListener> {
    let client = Client {
        ip: String::from("127.0.0.1"),
        port: 2525
    };

    let connection_string = format!("{}:{}", client.ip, client.port);

    TcpListener::bind(connection_string)
}

fn handle_incoming_connection(listener: TcpListener) -> std::io::Result<()> {
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    
    println!("New client connected: {}", stream.peer_addr()?);        

    stream.write_all(b"220 Service ready\r\n")?;

    let mut reader = BufReader::new(&stream);

    let mut line = String::new();

    loop {
        line.clear();

        let bytes_read = reader.read_line(&mut line)?;

        
        if bytes_read == 0 {
            stream.write_all(b"Connection terminated");

            println!("Client disconnected: {}", stream.peer_addr()?);
            break;
        }

        let command = line.trim_end();

        if line.is_empty() {
            continue;
        }
        
        println!("Received command: {}", command);
    }

    Ok(())
}




fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let listener = listen_client()?;

    handle_incoming_connection(listener)?;
    Ok(())
}
