mod cybernet;

use clap::Parser;
use cybernet::{client::Client, server::Server};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    test: String,
}

fn main() {
    let args = Args::parse();

    let test = Test {
        test: "me".to_string(),
    };

    if args.client {
        let client = Client {
            address: "127.0.0.1",
            port: "2000",
        };

        let data: Vec<u8> = bincode::serialize(&test).unwrap();

        let res: Vec<u8> = client.send(&data).unwrap();

        let test: Test = bincode::deserialize(&res[..]).unwrap();

        println!("{test:?}")
    }

    if args.server {
        let server = Server {
            address: "127.0.0.1",
            port: "2000",
        };

        let _ = server.start();
    }
}
