use std::str;
use std::{io::Result, net::UdpSocket};

use crate::Test;

pub struct Server {
    pub address: &'static str,
    pub port: &'static str,
}

impl Server {
    pub fn start(&self) -> Result<()> {
        let net = format!("{}:{}", self.address, self.port);

        let socket = UdpSocket::bind(net)?;

        let mut buffer = [0; 1024];

        loop {
            let (len, addr) = socket.recv_from(&mut buffer)?;

            let buf = &mut buffer[..len];

            socket.send_to(&buf, &addr)?;

            let test: Test = bincode::deserialize(&buf[..]).unwrap();

            println!("Reveived {test:?} from {addr:?}");
        }
    }
}
