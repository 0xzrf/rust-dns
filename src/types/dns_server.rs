use crate::{BUFFER_MAX_LEN, DnsErrors, DnsResult};
use std::net::UdpSocket;

pub struct DnsServer {
    pub socket: UdpSocket,
}

impl DnsServer {
    pub fn new(port: u64) -> DnsResult<Self> {
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", port))
            .map_err(|_| DnsErrors::UnableToBind { port })?;

        Ok(Self { socket })
    }

    pub fn handle_request(&self) -> DnsResult<()> {
        let mut buf = [0; BUFFER_MAX_LEN];
        let udp_socket = &self.socket;
        loop {
            match udp_socket.recv_from(&mut buf) {
                Ok((size, source)) => {
                    println!("Received {} bytes from {}", size, source);
                    let mut response = vec![];
                    let header: [u8; 12] = [0x04, 0xd2, 0x80, 0, 0, 1, 0, 1, 0, 0, 0, 0];
                    let question = b"\x0ccodecrafters\x02io\x00\x00\x01\x00\x01";
                    let answer =
                        b"\x0ccodecrafters\x02io\x00\x00\x01\x00\x01\x00\x01\x00\x01\x00\x04\x08\x08\x08\x08";

                    response.extend_from_slice(&header);
                    response.extend_from_slice(question);
                    response.extend_from_slice(answer);

                    udp_socket
                        .send_to(&response, source)
                        .expect("Failed to send response");
                }
                Err(e) => {
                    eprintln!("Error receiving data: {}", e);
                    return Err(DnsErrors::ErrorReceivingPacket {
                        error: e.to_string(),
                    });
                }
            }
        }
    }
}

#[cfg(test)]
pub mod dns_server_tests {
    use crate::constants::UDP_PORT;

    use super::*;
    #[test]
    pub fn test_creates_new_dns() {
        let create_dns_result = DnsServer::new(UDP_PORT);

        assert!(
            create_dns_result.is_ok(),
            "Expected dns server to be created for port: {}",
            UDP_PORT
        );

        let invalid_port = 10;
        let fails_on_invalid_port_result = DnsServer::new(invalid_port); // invalid port since it's a priviledged port

        assert!(
            fails_on_invalid_port_result.is_err(),
            "Expected dns server to fail on port: {}",
            invalid_port
        );

        if let Some(err) = fails_on_invalid_port_result.err() {
            assert_eq!(
                err,
                DnsErrors::UnableToBind { port: invalid_port },
                "The error did not match"
            );
        }
    }
}
