pub mod dns_server;
pub use dns_server::DnsServer;

pub mod sections;
pub use sections::{Header, Question};

mod dns_message;
pub mod header;
