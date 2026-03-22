use crate::{DnsErrors, DnsResult};
use std::net::UdpSocket;

pub struct DnsServer {
    pub socket: UdpSocket,
}

impl DnsServer {
    pub fn new(port: u64) -> DnsResult<Self> {
        let socket =
            UdpSocket::bind(format!("127.0.0.1:{}", port)).map_err(|_| DnsErrors::UnableToBind)?;

        Ok(Self { socket })
    }
}
