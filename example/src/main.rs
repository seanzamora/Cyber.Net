extern crate cybernet;

use clap::Parser;
use cybernet::{Bincode, Client, Message, MessageAction, Server};
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

    if args.client {
        let client = Client {
            address: "127.0.0.1",
            port: "2000",
        };

        let msg = Message::<Test> {
            action: MessageAction::Movement,
            data: Test {
                test: "me".to_string(),
            },
        };

        let mut x = 0;
        loop {
            let res: Vec<u8> = client.send(&msg).unwrap();
            let dec: Message<Test> = Message::deserialize(res);
            x += 1;
            println!("{dec:?}{x:?}")
        }
    }

    if args.server {
        let server = Server {
            address: "127.0.0.1",
            port: "2000",
        };

        let _ = server.start();
    }
}
