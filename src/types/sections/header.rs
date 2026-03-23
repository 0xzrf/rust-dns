use crate::errors::{DnsErrors, DnsResult};
use bytes::Buf;

pub struct Header([u8; 12]);

impl Header {
    pub fn new(mut data: &[u8]) -> DnsResult<Header> {
        let mut header_inner = Vec::with_capacity(12);
        for _ in 0..12 {
            let header_data = data.get_u8();
            header_inner.push(header_data);
        }

        if data.len() != 12 {
            return Err(DnsErrors::InvalidQuestionSection {
                reason: "Invalid data len".to_string(),
            });
        }

        Ok(Self(data.try_into().unwrap()))
    }
}
