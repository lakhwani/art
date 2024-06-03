#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{Art, State, BALANCES, GALLERY, OWNERS, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmwasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
        art_counter: 0u64,
        royalty_rate: msg.royalty_rate,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string())
        .add_attribute("art_counter", msg.count.to_string())
        .add_attribute("royalty_rate", msg.royalty_rate.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => execute::increment(deps),
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
        ExecuteMsg::Deposit {} => execute::deposit(deps, info),
        ExecuteMsg::Withdraw { amount } => execute::withdraw(deps, info, amount),
        ExecuteMsg::CreateArt { price, rfid } => execute::create(deps, info, price, rfid),
        ExecuteMsg::PurchaseArt { art_id } => execute::purchase(deps, info, art_id),
    }
}

pub mod execute {

    use cosmwasm_std::{Coin, Uint128};

    use super::*;

    pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }

    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }

    pub fn deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let sender = info.sender.clone();
        let mut balance = BALANCES
            .may_load(deps.storage, sender.clone())?
            .unwrap_or_default();

        for coin in info.funds {
            if coin.denom == "ucosm" {
                balance += coin.amount;
            }
        }

        if balance.is_zero() {
            return Err(ContractError::EmptyBalance {});
        }

        BALANCES.save(deps.storage, sender, &balance)?;

        Ok(Response::new()
            .add_attribute("action", "deposit")
            .add_attribute("account", info.sender.to_string())
            .add_attribute("amount", balance.clone()))
    }

    pub fn withdraw(
        deps: DepsMut,
        info: MessageInfo,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let sender = info.sender.clone();
        let balance = BALANCES
            .may_load(deps.storage, sender.clone())?
            .unwrap_or_default();

        if amount > balance {
            return Err(ContractError::InsufficientBalance {});
        }

        Ok(Response::new()
            .add_attribute("action", "withdraw")
            .add_attribute("sender", sender.clone())
            .add_message(BankMsg::Send {
                to_address: sender.into(),
                amount: vec![Coin {
                    denom: "ucosm".to_string(),
                    amount,
                }],
            }))
    }

    pub fn create(
        deps: DepsMut,
        info: MessageInfo,
        price: Uint128,
        rfid: u64,
    ) -> Result<Response, ContractError> {
        let owner = STATE.load(deps.storage)?.owner;
        let sender = info.sender.clone();
        if owner == sender {
            return Err(ContractError::Unauthorized {});
        }

        let mut state = STATE.load(deps.storage)?;
        let art_id: u64 = state.art_counter;
        let art_data: Art = Art {
            art_id,
            price,
            rfid,
        };

        GALLERY.save(deps.storage, art_id, &art_data)?;
        OWNERS.save(deps.storage, art_id, &sender)?;

        state.art_counter += 1;
        STATE.save(deps.storage, &state)?;

        Ok(Response::new()
            .add_attribute("action", "create")
            .add_attribute("art_id", art_id.to_string()))
    }

    pub fn purchase(
        deps: DepsMut,
        info: MessageInfo,
        art_id: u64,
    ) -> Result<Response, ContractError> {
        let sender = info.sender.clone();
        let mut balance = BALANCES
            .may_load(deps.storage, sender.clone())?
            .unwrap_or_default();
        let art = GALLERY.may_load(deps.storage, art_id)?.unwrap();

        if balance < art.price {
            return Err(ContractError::InsufficientBalance {});
        }

        balance -= art.price;
        BALANCES.save(deps.storage, sender.clone(), &balance)?;
        OWNERS.save(deps.storage, art_id, &sender)?;

        Ok(Response::new()
            .add_attribute("action", "purchase")
            .add_attribute("art_id", art_id.to_string()))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&query::count(deps)?),
        QueryMsg::GetArt { art_id } => to_json_binary(&query::art(deps, art_id)?),
        QueryMsg::GetArtOwner { art_id } => to_json_binary(&query::artowner(deps, art_id)?),
        QueryMsg::GetBalance { addr } => to_json_binary(&query::balance(deps, addr)?),
    }
}

pub mod query {
    use crate::msg::{GetArtOwnerResponse, GetArtResponse, GetBalanceResponse};
    use cosmwasm_std::Addr;

    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }

    pub fn art(deps: Deps, art_id: u64) -> StdResult<GetArtResponse> {
        let art = GALLERY.may_load(deps.storage, art_id)?.unwrap();
        Ok(GetArtResponse { art })
    }

    pub fn artowner(deps: Deps, art_id: u64) -> StdResult<GetArtOwnerResponse> {
        let owner = OWNERS.may_load(deps.storage, art_id)?.unwrap();
        Ok(GetArtOwnerResponse { owner })
    }

    pub fn balance(deps: Deps, addr: Addr) -> StdResult<GetBalanceResponse> {
        let balance = BALANCES.may_load(deps.storage, addr)?.unwrap_or_default();
        Ok(GetBalanceResponse { balance })
    }
}

#[cfg(test)]
mod tests {
    use crate::msg::GetBalanceResponse;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json, Addr, Uint128};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            count: 17,
            royalty_rate: 5u64,
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            count: 17,
            royalty_rate: 5u64,
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            count: 17,
            royalty_rate: 5u64,
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(5, value.count);
    }

    #[test]
    fn deposit() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            count: 0,
            royalty_rate: 5u64,
        };
        let creator = Addr::unchecked("creator");
        let info = mock_info(&creator.to_string(), &coins(1000, "ucosm"));

        // Instantiate the contract
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // Deposit funds
        let depositor = Addr::unchecked("depositor");
        let deposit_info = mock_info(&depositor.to_string(), &coins(500, "ucosm"));
        let msg = ExecuteMsg::Deposit {};
        let res = execute(deps.as_mut(), mock_env(), deposit_info.clone(), msg).unwrap();

        // Check if the deposit action was successful
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "deposit");
        assert_eq!(res.attributes[1].key, "account");
        assert_eq!(res.attributes[1].value, depositor.to_string());

        // Query balance to ensure it is updated
        let balance_query = QueryMsg::GetBalance {
            addr: Addr::unchecked("depositor"),
        };
        let res = query(deps.as_ref(), mock_env(), balance_query).unwrap();
        let value: GetBalanceResponse = from_json(&res).unwrap();
        assert_eq!(Uint128::from(500u128), value.balance);
    }
}
