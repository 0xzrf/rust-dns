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
        let query_header = &self.header;
        let query_question = &self.question;

        let header_response = Header::new()
            .with_id(query_header.id())
            .with_qr(true)
            .with_opcode(query_header.opcode())
            .with_aa(false)
            .with_tc(false)
            .with_rd(query_header.rd())
            .with_ra(false)
            .with_z(0)
            .with_rcode(if query_header.opcode() == 0 { 0 } else { 4 })
            .with_qdcount(1)
            .with_ancount(1)
            .with_nscount(1)
            .with_arcount(1);

        let mut question_response: Vec<u8> = vec![];
        question_response.extend(query_question.second_ld.clone()); // mimic the labels
        question_response.extend(query_question.top_ld.clone());
        question_response.extend(0x00u8.to_le_bytes()); // add null byte
        question_response.extend((1u16).to_be_bytes());
        question_response.extend((1u16).to_be_bytes());

        let mut answer_response: Vec<u8> = vec![];
        answer_response.extend(query_question.second_ld.clone()); // mimic the labels
        answer_response.extend(query_question.top_ld.clone());
        answer_response.extend((1u16).to_be_bytes()); // "A" record type
        answer_response.extend((1u16).to_be_bytes()); // "IN" record class
        answer_response.extend((60u32).to_be_bytes()); // TTL
        answer_response.extend((4u16).to_be_bytes()); // length of RDATA
        answer_response.extend(b"\x08\x08\x08\x08");

        let mut response = header_response.into_bytes().to_vec();
        response.extend(question_response);
        response.extend(answer_response);

        Ok(response)
    }
}
