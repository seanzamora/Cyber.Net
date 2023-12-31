use std::str;
use std::{io::Result, net::UdpSocket};

pub struct Client {
    pub address: &'static str,
    pub port: &'static str,
}

impl Client {
    pub fn send(&self, line: &[u8]) -> Result<Vec<u8>> {
        let net = format!("{}:{}", self.address, self.port);

        let socket = UdpSocket::bind("[::]:0")?;

        socket.send_to(line, net).expect("Error on send");

        let mut buf = [0; 2048];

        let (len, _src) = socket.recv_from(&mut buf)?;

        Ok(buf[..len].to_vec())
    }
}
