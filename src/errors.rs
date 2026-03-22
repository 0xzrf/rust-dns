use thiserror::Error;

pub type DnsResult<T> = Result<T, DnsErrors>;

#[derive(Error, Debug)]
pub enum DnsErrors {
    #[error("Unable to connect to port")]
    UnableToBind,
}
