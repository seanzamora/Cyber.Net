use derive_macro::Bincode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Bincode)]
pub struct Message<T> {
    r#type: MessageType,
    data: T,
}

#[derive(Serialize, Deserialize, Debug, Bincode)]
pub enum MessageType {
    Action,
    Movement,
    Inventory,
    Transaction,
    Custom,
}
