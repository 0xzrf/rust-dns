use bytes::Buf;

use crate::{DnsErrors, DnsResult};
#[derive(Debug)]
pub struct Question {
    pub labels: Vec<(usize, Vec<u8>)>,
    pub q_type: i16,
    pub class: i16,
}

impl Question {
    /// Creates a new Question struct
    ///
    /// Panics if data.get_u8() fails
    pub fn new(data: &[u8]) -> DnsResult<Self> {
        let mut data_ix = 0usize;

        let mut labels = vec![];

        while data[data_ix] != 0 {
            let label_len = data[data_ix] as usize;

            let label_data = data
                .get(data_ix + 1..(data_ix + 1 + label_len))
                .unwrap()
                .to_vec();

            labels.push((label_len, label_data));

            data_ix += label_len + 1;
        }

        let mut remaining_metadata = data.get(data_ix + 1..).unwrap();

        let q_type = remaining_metadata.get_i16();
        let class = remaining_metadata.get_i16();

        Ok(Question {
            labels,
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
                for ((_, label), expected) in question.labels.iter().zip(["codecrafters", "io"]) {
                    assert_eq!(*label, expected.as_bytes(), "labels don't match");
                }

                let class = question.class;
                let q_type = question.q_type;

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
    pub fn test_passes_if_multiple_lable_correct() {
        let data = b"\x04mail\x0ccodecrafters\x02io\x00\x00\x01\x00\x01";

        match Question::new(data) {
            Ok(question) => {
                for ((_, label), expected) in
                    question.labels.iter().zip(["mail", "codecrafters", "io"])
                {
                    assert_eq!(*label, expected.as_bytes(), "labels don't match");
                }

                let class = question.class;
                let q_type = question.q_type;

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
}
