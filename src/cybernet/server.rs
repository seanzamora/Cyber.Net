use std::str;
use std::{io::Result, net::UdpSocket};

pub struct Server {
    pub address: &'static str,
    pub port: &'static str,
}

impl Server {
    pub fn start(&self) -> Result<()> {
        let net = format!("{}:{}", self.address, self.port);

        let socket = UdpSocket::bind(net)?;

        let mut buf = [0; 1024];

        loop {
            let (len, addr) = socket.recv_from(&mut buf)?;

            let buf = &mut buf[..len];

            socket.send_to(&buf, &addr)?;

            println!("{:?} received from {:?}.", len, addr);
            println!("{:?} data received.", str::from_utf8(&buf[..len]));
        }
    }
}
