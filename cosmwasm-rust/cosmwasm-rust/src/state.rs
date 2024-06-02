use cosmwasm_std::Storage;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Art {
    pub artId: u64,
    pub price: u64;
    pub rfid: u64;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub artCounter: u64,
    pub royaltyRate: u64,
}

pub const STATE: Item<State> = Item::new("state");
pub const GALLERY: Map<u64, Art> = Map::new("gallery");
pub const OWNERS: Map<u64, Addr> = Map::new("owners");
