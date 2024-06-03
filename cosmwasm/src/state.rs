use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Art {
    pub art_id: u64,
    pub price: Uint128,
    pub rfid: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
    pub art_counter: u64,
    pub royalty_rate: u64,
}

pub const STATE: Item<State> = Item::new("state");
pub const GALLERY: Map<u64, Art> = Map::new("gallery");
pub const OWNERS: Map<u64, Addr> = Map::new("owners");
pub const BALANCES: Map<Addr, Uint128> = Map::new("balances");
