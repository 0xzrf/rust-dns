mod constants;
mod errors;
mod types;
use constants::*;
use errors::*;

use constants::UDP_PORT;
use types::DnsServer;

pub fn run_dns_server() -> DnsResult<()> {
    let dns_socket_handler = DnsServer::new(UDP_PORT);

    Ok(())
}
