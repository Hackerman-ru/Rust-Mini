#![allow(unused)]

use anyhow::Result;
use core::time;
use rand::Rng;
use std::{
    io::{self, Read, Write},
    net::{IpAddr, Shutdown, SocketAddr, TcpStream},
    process::{Child, Command},
    str::FromStr,
};
use std::{str, thread};

pub enum IpVersion {
    V4,
    V6,
}

pub struct Server {
    proc: Option<Child>,
    pub addr: SocketAddr,
}

impl Server {
    pub fn start(ip_version: IpVersion) -> Self {
        let mut generator = rand::thread_rng();
        let port: u16 = generator.gen_range(40000..49151);
        let ip = match ip_version {
            IpVersion::V4 => IpAddr::from_str("127.0.0.1").unwrap(),
            IpVersion::V6 => IpAddr::from_str("::1").unwrap(),
        };
        let proc = Command::new("cargo")
            .args([
                "run",
                "--",
                "--ip",
                &ip.to_string(),
                "--port",
                &port.to_string(),
            ])
            .spawn()
            .expect("can't start server");
        thread::sleep(time::Duration::from_secs(1));
        Self {
            proc: Some(proc),
            addr: SocketAddr::new(ip, port),
        }
    }
    pub fn is_alive(&mut self) -> bool {
        self.proc
            .as_mut()
            .map_or(false, |proc| proc.try_wait().unwrap().is_none())
    }
    pub fn stop(&mut self) {
        let _ = self.proc.take().map_or(Ok(()), |mut proc| proc.kill());
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop();
    }
}

pub struct Client {
    connection: TcpStream,
    delimiter: u8,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            connection: self
                .connection
                .try_clone()
                .expect("can't clone tcp connection"),
            delimiter: self.delimiter,
        }
    }
}

impl Client {
    pub fn start(server_addr: SocketAddr) -> Result<Self> {
        let connection = TcpStream::connect(server_addr)?;
        Ok(Self {
            connection,
            delimiter: b'\n',
        })
    }

    pub fn set_delimeter(&mut self, delimeter: u8) -> &mut Self {
        self.delimiter = delimeter;
        self
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.connection
            .write_all(&[data, &[self.delimiter]].concat())
    }

    pub fn read_expect(&mut self, expected: &[u8]) {
        let mut buf = vec![0; expected.len() + 1];
        self.connection.read_exact(&mut buf).unwrap();
        assert_eq!(expected, &buf[..expected.len()]);
        assert_eq!(self.delimiter, buf[expected.len()]);
    }

    pub fn read_expect_nothing(&self) -> io::Result<()> {
        self.connection.set_nonblocking(true)?;
        let mut buf = [0; 1];
        let is_empty = match self.connection.peek(&mut buf) {
            Ok(n) => n == 0,
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => true,
            Err(err) => {
                self.connection.set_nonblocking(false)?;
                return Err(err);
            }
        };
        self.connection.set_nonblocking(false)?;
        assert!(is_empty);
        Ok(())
    }

    pub fn shutdown(&mut self, how: Shutdown) {
        let _ = self.connection.shutdown(how);
    }
}
