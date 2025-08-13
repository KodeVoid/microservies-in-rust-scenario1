use std::net::{TcpStream,Shutdown};
use std::io::{Read,Write};
use std::str;

fn main() {
    let  mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write_all("Hello world".as_bytes()).unwrap();

    let mut buffer = [0;1024];
  let bytes_read =  stream.read(&mut buffer).unwrap();

    println!("Got response from server {:?}", 
        str::from_utf8(&buffer[..bytes_read]).unwrap());
    stream.write_all("Closing connection".as_bytes()).unwrap();

let bytes_read   = stream.read(&mut buffer).unwrap();
    println!("Got response from server {:?}", 
        str::from_utf8(&buffer[..bytes_read]).unwrap());

    stream.shutdown(Shutdown::Both).unwrap();


}