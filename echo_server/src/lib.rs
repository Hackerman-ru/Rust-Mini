#![forbid(unsafe_code)]

use std::{
    io::{BufRead, BufReader, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    thread,
};

fn handle_client(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    let mut reader = BufReader::new(stream.try_clone()?);
    loop {
        let mut buf: Vec<u8> = vec![];
        let _ = reader.read_until(b'\n', &mut buf);
        stream.write_all(buf.as_mut())?;
    }
}

pub fn run(ip: IpAddr, port: u16) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(SocketAddr::new(ip, port))?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let handler = |mut stream: TcpStream| move || handle_client(&mut stream);
        thread::spawn(handler(stream));
    }
    Ok(())
}
