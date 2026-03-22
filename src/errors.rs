use anyhow::Result;
use thiserror::Error;

pub type DnsResult<T> = Result<T, DnsErrors>;

#[derive(Error, Debug, PartialEq)]
pub enum DnsErrors {
    #[error("Unable to connect to udp port: {port}")]
    UnableToBind { port: u64 },
}
