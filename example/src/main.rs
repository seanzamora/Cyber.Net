extern crate cybernet;

use clap::Parser;
use cybernet::{Bincode, Client, Server};
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

    // let test = Test {
    //     test: "me".to_string(),
    // };

    if args.client {
        let _client = Client {
            address: "127.0.0.1",
            port: "2000",
        };

        // loop {
        //     let res: Vec<u8> = client.send(&test.serialize()).unwrap();
        //     let dec: Test = Test::deserialize(res);
        //     println!("{dec:?}")
        // }
    }

    if args.server {
        let server = Server {
            address: "127.0.0.1",
            port: "2000",
        };

        let _ = server.start();
    }
}
