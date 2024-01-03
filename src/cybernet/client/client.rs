use std::str;
use std::{io::Result, net::UdpSocket};

use serde::{Deserialize, Serialize};

use crate::Message;

pub struct Client {
    pub address: &'static str,
    pub port: &'static str,
}

impl Client {
    pub fn send<T: for<'a> Deserialize<'a> + Serialize>(
        &self,
        message: &Message<T>,
    ) -> Result<Vec<u8>> {
        let data = &message.serialize();

        let net = format!("{}:{}", self.address, self.port);

        let socket = UdpSocket::bind("[::]:0")?;

        socket.send_to(data, net).expect(
            format!(
                "Error sending message {:?} to {:?}:{:?}",
                data, self.address, self.port
            )
            .as_str(),
        );

        let mut buf = [0; 255usize];

        let (len, _src) = socket.recv_from(&mut buf)?;

        Ok(buf[..len].to_vec())
    }
}
