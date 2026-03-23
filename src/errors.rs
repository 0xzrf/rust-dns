use thiserror::Error;

pub type DnsResult<T> = Result<T, DnsErrors>;

#[derive(Error, Debug, PartialEq)]
pub enum DnsErrors {
    #[error("Unable to connect to udp port: {port}")]
    UnableToBind { port: u64 },

    #[error("Error receiving packet: {error}")]
    ErrorReceivingPacket { error: String },

    #[error("Invalid Question section format")]
    InvalidQuestionSection,
}

impl DnsErrors {
    pub fn error_msg(&self) -> String {
        self.to_string()
    }
}
