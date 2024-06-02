use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]

pub struct Art {
    pub art_id: u64,
    pub price: u64,
    pub rfid: u64,
}

pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
pub const GALLERY: Map<u64, Art> = Map::new("gallery");
pub const OWNERS: Map<u64, Addr> = Map::new("owners");
