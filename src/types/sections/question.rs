use bytes::Buf;

use crate::{DnsErrors, DnsResult};

pub struct Question {
    pub second_ld: Vec<u8>,
    pub top_ld: Vec<u8>, // ld== level domain
    pub q_type: i16,
    pub class: i16,
}

impl Question {
    /// Creates a new Question struct
    ///
    /// Panics if data.get_u8() fails
    pub fn new(mut data: &[u8]) -> DnsResult<Self> {
        let second_ld_len = data.get_u8() as usize;

        let mut second_ld = vec![];

        for _ in 0..second_ld_len {
            let second_ld_data = data.get_u8();
            second_ld.push(second_ld_data);
        }

        let top_ld_len = data.get_u8() as usize;

        let mut top_ld = vec![];
        for _ in 0..top_ld_len {
            let top_ld_data = data.get_u8();
            top_ld.push(top_ld_data);
        }

        if data.get_u8() != 0x00 {
            return Err(DnsErrors::InvalidQuestionSection {
                reason: "Invalid null byte".to_string(),
            });
        }

        let q_type = data.get_i16().to_le();
        let class = data.get_i16().to_le();

        Ok(Self {
            second_ld,
            top_ld,
            q_type,
            class,
        })
    }
}

#[cfg(test)]
pub mod test_question_section {
    use super::*;

    #[test]
    pub fn test_passes_if_data_correct() {
        let data = b"\x0ccodecrafters\x02io\x00\x00\x01\x00\x01";

        match Question::new(data) {
            Ok(question) => {
                println!("Question::new ran successfully");

                let top_ld = question
                    .top_ld
                    .iter()
                    .map(|b| *b as char)
                    .collect::<String>();
                let second_ld = question
                    .second_ld
                    .iter()
                    .map(|b| *b as char)
                    .collect::<String>();
                let class = question.class;
                let q_type = question.q_type;

                println!("top ld: {top_ld}\nsecond_ld: {second_ld}");

                assert_eq!(
                    "codecrafters".to_string(),
                    second_ld,
                    "Invalid second ld val"
                );
                assert_eq!("io".to_string(), top_ld, "Invalid second ld val");

                assert_eq!(q_type, 1, "q_type has invalid value");
                assert_eq!(class, 1, "class has invalid value ");
            }
            Err(e) => {
                println!("Expected the procedure to run successfully: {e:#?}");
                panic!();
            }
        }
    }

    #[test]
    pub fn test_fail_on_invalid_data() {
        let data = b"\x06google\x03com\x01\x01\x00\x01";
        let question_create_result = Question::new(data);
        assert!(question_create_result.is_err(), "expected this to fail");
        assert_eq!(
            DnsErrors::InvalidQuestionSection {
                reason: "Invalid null byte".to_string()
            },
            question_create_result.err().unwrap()
        );
    }

    #[test]
    pub fn something() {}
}
