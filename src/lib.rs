#[warn(dead_code)]
mod cybernet;

pub use cybernet::{Client, Message, MessageAction, Server};
pub use derive_macro::Bincode;
