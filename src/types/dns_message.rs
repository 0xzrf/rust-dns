use crate::errors::DnsResult;
use crate::{Header, types::Question};

pub struct DnsMessage {
    header: Header,
    question: Question,
}

impl DnsMessage {
    pub fn build_message(query: &[u8]) -> DnsResult<Self> {
        let header = Header::new(query)?;
        let question = Question::new(query)?;

        Ok(DnsMessage { header, question })
    }
}
