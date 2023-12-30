use std::str;
use std::{io::Result, net::UdpSocket};

pub struct Client {
    pub address: &'static str,
    pub port: &'static str,
}

impl Client {
    pub fn send(&self, line: &'static str) -> Result<()> {
        let net = format!("{}:{}", self.address, self.port);

        let socket = UdpSocket::bind("[::]:0")?;

        socket.send_to(line.as_bytes(), net).expect("Error on send");

        let mut buf = [0; 2048];

        let (len, _src) = socket.recv_from(&mut buf)?;

        println!("Echo {:?}", str::from_utf8(&buf[..len]).unwrap());

        Ok(())
    }
}
