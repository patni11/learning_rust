use smtp_server::server;

fn main() -> std::io::Result<()> {
    server::run_server()
}
