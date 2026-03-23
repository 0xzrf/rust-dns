use bytes::Buf;

use crate::{DnsErrors, DnsResult};

pub struct Question {
    second_ld: String,
    top_ld: String, // ld== level domain
    q_type: i16,
    class: i16,
}

impl Question {
    pub fn new(data: &[u8]) -> DnsResult<Self> {
        let second_ld_len = *data.first().ok_or(DnsErrors::InvalidQuestionSection)? as usize;

        let second_ld = data
            .iter()
            .skip(1)
            .take(second_ld_len)
            .map(|b| *b as char)
            .collect::<String>();

        let top_ld_len = data[1 + (second_ld_len)] as usize; // should pass if the question section of dns is correct

        let top_ld = data
            .iter()
            .skip(2 + (second_ld_len))
            .take(top_ld_len)
            .map(|b| *b as char)
            .collect::<String>();

        let mut remaining_metadata = data
            .get(2 + second_ld_len + top_ld_len..)
            .ok_or(DnsErrors::InvalidQuestionSection)?;

        let q_type = remaining_metadata.get_i16();
        let class = remaining_metadata.get_i16();

        Ok(Self {
            second_ld,
            top_ld,
            q_type,
            class,
        })
    }
}
