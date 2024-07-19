#![allow(unused)]

use net_framework::*;
use ntest::timeout;
use std::{net::IpAddr, str::FromStr};

macro_rules! send_and_check {
    ($msg:expr, $server:expr) => {
        let mut client = Client::start($server.addr).unwrap();
        client.write($msg);
        assert!($server.is_alive());
        client.read_expect($msg);
    };
}

#[test]
#[timeout(2000)]
fn test_simple() {
    let mut server = Server::start(IpVersion::V4);
    assert!(server.is_alive());
    send_and_check!(b"Hello, world!", server);
    assert!(server.is_alive());
}

#[test]
#[timeout(2000)]
fn test_many() {
    let mut server = Server::start(IpVersion::V4);
    assert!(server.is_alive());
    send_and_check!(b"Hello, world 1!", server);
    send_and_check!(b"Hello, world 2!", server);
    send_and_check!(b"Hello, world 3!", server);
    send_and_check!(b"Hello, world 2024!", server);
    assert!(server.is_alive());
}

#[test]
#[timeout(2000)]
fn test_non_utf8() {
    let mut server = Server::start(IpVersion::V4);
    assert!(server.is_alive());
    send_and_check!(&[255, 12, 22, 33, 44], server);
    assert!(server.is_alive());
}

#[test]
#[timeout(2000)]
fn test_stress() {
    let mut server = Server::start(IpVersion::V4);
    assert!(server.is_alive());
    let msg = &[228; 1000];
    let mut client = Client::start(server.addr).unwrap();
    for _ in 0..1000 {
        client.write(msg);
    }
    assert!(server.is_alive());
    for _ in 0..1000 {
        client.read_expect(msg);
    }
    assert!(server.is_alive());
    for _ in 0..1000 {
        client.read_expect_nothing();
    }
    assert!(server.is_alive());
}
