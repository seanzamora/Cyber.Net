mod cybernet;
use clap::Parser;
use cybernet::{Client, Server};
use derive_macro::Bincode;
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

#[derive(Serialize, Deserialize, Debug, Bincode)]
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

        let res: Vec<u8> = client.send(&test.serialize()).unwrap();
        let dec: Test = Test::deserialize(res);

        println!("{dec:?}")
    }

    if args.server {
        let server = Server {
            address: "127.0.0.1",
            port: "2000",
        };

        let _ = server.start();
    }
}
