use crate::errors::DnsResult;
use crate::{Header, types::Question};
use bytes::{Buf, BufMut, Bytes, BytesMut};

pub struct DnsMessage {
    header: Header,
    question: Question,
}

impl DnsMessage {
    pub fn build_message(query: &[u8]) -> DnsResult<Self> {
        let header = &query[0..12];

        todo!()
    }
}
