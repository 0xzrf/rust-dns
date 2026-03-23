use codecrafters_dns_server::run_dns_server;

fn main() {
    if let Err(e) = run_dns_server() {
        eprint!("{}", e.error_msg());
        std::process::exit(1);
    }

    std::process::exit(0);
}
