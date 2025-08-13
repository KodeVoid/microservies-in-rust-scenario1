use std::net::TcpListener;
use std::io::{Read,Write};
use std::str;

fn main() {
    let connection_listener=TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");

    for stream in connection_listener.incoming(){
        let mut stream = stream.unwrap();
        println!("Connection Established");

        let mut buffer = [0;1024];

        let bytes_read =stream.read(&mut buffer).unwrap();
        println!("Client sent {:?}",
            str::from_utf8(&buffer[..bytes_read] ).unwrap());

        stream.write_all(&mut buffer[..bytes_read]).unwrap();

       let bytes_read= stream.read(&mut buffer).unwrap();
        println!("Client sent {:?}",
            str::from_utf8(&buffer[..bytes_read] ).unwrap());
        stream.write_all("Acknowledged".as_bytes()).unwrap();


    }
}
