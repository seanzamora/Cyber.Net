use std::i64;

use derive_macro::Bincode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Message<T> {
    pub action: MessageAction,
    pub data: T,
}

impl<T: for<'a> Deserialize<'a> + Serialize> Message<T> {
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn deserialize(input: Vec<u8>) -> Self {
        bincode::deserialize::<Message<T>>(&input[..]).unwrap()
    }
}

#[derive(Deserialize, Serialize, Bincode, Debug)]
pub enum MessageAction {
    Movement,
    Inventory,
    Trade,
    Custom,
}

struct Entity {
    pub id: i64,
    pub kind: EntityKind,
    pub faction: Faction,
    pub class: EntityClass,
    pub race: Race,
}

struct Race {
    pub id: i64,
    pub kind: RaceKind,
    pub name: String,
    pub description: String,
}

enum RaceKind {
    Human,
    Bear,
}

struct EntityClass {
    pub id: i64,
    pub kind: ClassKind,
}

enum ClassKind {
    Warrior,
    Rouge,
    Hunter,
}
struct Faction {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub kind: FactionKind,
}

enum FactionKind {
    One,
    Two,
    Three,
}

enum EntityKind {
    Player,
    Vendor,
    NonPlayer,
    Boss,
}

struct EntityStats {
    pub entity: Entity,
    pub metric: StatMetric,
    pub current: i64,
    pub max: i64,
}

enum StatMetric {
    Mana,
    Health,
    Rage,
    Energy,
    Strength,
    Armor,
    CriticalStike,
    SpellPower,
    HitRating,
}

struct Movement {
    pub entity: Entity,
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
}

struct Item {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub icon: String,
}

struct Trade {
    pub items: Vec<TradeItem>,
    pub players: Vec<Entity>,
}

struct TradeItem {
    pub item: Item,
    pub from: Entity,
    pub to: Entity,
}

struct Inventory {
    pub player: Entity,
    pub items: Vec<InventoryItem>,
}

struct InventoryItem {
    pub item: Item,
    pub slot_id: i64,
}
