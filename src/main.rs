mod cybernet;

use clap::Parser;
use cybernet::{client::Client, server::Server};

#[derive(Parser, Debug)]
#[clap(
    author = "Sean Zamora",
    version,
    about = "CyberNet game server framework."
)]
struct Args {
    #[clap(long, default_value_if("", "true", Some("true")))]
    client: bool,

    #[clap(long, default_value_if("", "true", Some("true")))]
    server: bool,
}

fn main() {
    let args = Args::parse();

    if args.client {
        let client = Client {
            address: "127.0.0.1",
            port: "2000",
        };

        let _ = client.send("Testing");
    }

    if args.server {
        let server = Server {
            address: "127.0.0.1",
            port: "2000",
        };

        let _ = server.start();
    }
}
