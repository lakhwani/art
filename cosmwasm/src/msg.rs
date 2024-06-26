use crate::state::Art;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
    pub royalty_rate: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    CreateArt { price: Uint128, rfid: u64 },
    PurchaseArt { art_id: u64 },
    Deposit {},
    Withdraw { amount: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
    #[returns(GetArtResponse)]
    GetArt { art_id: u64 },
    #[returns(GetCountResponse)]
    GetArtOwner { art_id: u64 },
    #[returns(GetBalanceResponse)]
    GetBalance { addr: Addr },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

#[cw_serde]
pub struct GetArtResponse {
    pub art: Art,
}

#[cw_serde]
pub struct GetArtOwnerResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct GetBalanceResponse {
    pub balance: Uint128,
}
