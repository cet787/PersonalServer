use std::io;
use std::io::{Write, Read};
use std::net::TcpStream;
use PersonalServer::tcp_message::{MultiField, TcpMessage};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let tcp_message = TcpMessage::SensorValueFloatMessage(45.6);
    let tcp_message_2 = TcpMessage::MultiFieldMessage(MultiField {
        name: "Connor".to_string(),
        value: 12,
    });

    stream.write_all(&tcp_message_2.encode())?;
    stream.write_all(&tcp_message.encode())?;

    println!("Sent TcpMessage");

    let mut buffer = [0u8; 1024];

    loop {
        stream.read(&mut buffer)?;
        println!("{}", String::from_utf8_lossy(&buffer));
    }
}