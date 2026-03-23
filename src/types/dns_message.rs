use crate::errors::DnsResult;
use crate::{Header, types::Question};

pub struct DnsMessage {
    header: Header,
    question: Question,
}

impl DnsMessage {
    pub fn build_query(query: &[u8]) -> DnsResult<Self> {
        let header = Header::from_bytes(query[..12].try_into().unwrap());

        let question = Question::new(&query[12..])?;

        Ok(DnsMessage { header, question })
    }

    pub fn build_response(&self) -> DnsResult<Vec<u8>> {
        let request = &self.header;

        let header_response = Header::new()
            .with_id(request.id())
            .with_qr(true)
            .with_opcode(request.opcode())
            .with_aa(false)
            .with_tc(false)
            .with_rd(request.rd())
            .with_ra(false)
            .with_z(0)
            .with_rcode(if request.opcode() == 0 { 0 } else { 4 })
            .with_qdcount(1)
            .with_ancount(1)
            .with_nscount(1)
            .with_arcount(1);

        let mut question: Vec<u8> = b"\x0ccodecrafters\x02io\x00".to_vec();
        question.extend((1u16).to_be_bytes());
        question.extend((1u16).to_be_bytes());

        let mut answer: Vec<u8> = b"\x0ccodecrafters\x02io\x00".to_vec();
        answer.extend((1u16).to_be_bytes()); // "A" record type
        answer.extend((1u16).to_be_bytes()); // "IN" record class
        answer.extend((60u32).to_be_bytes()); // TTL
        answer.extend((4u16).to_be_bytes()); // length of RDATA
        answer.extend(b"\x08\x08\x08\x08");

        let mut response = header_response.into_bytes().to_vec();
        response.extend(question);
        response.extend(answer);

        Ok(response)
    }
}
