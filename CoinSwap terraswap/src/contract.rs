#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, WasmMsg,
};
use cw2::set_contract_version;
use terraswap::asset::{Asset, AssetInfo};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::State;

use terra_cosmwasm::{create_swap_msg, TerraMsgWrapper};
use terraswap::pair::{Cw20HookMsg as TerraswapCw20HookMsg, ExecuteMsg as TerraswapExecuteMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:terraswap_pj";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<TerraMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::TerraSwapMsg {} => try_terra_swap(deps),
    }
}

pub fn try_terra_swap(deps: DepsMut) -> Result<Response<TerraMsgWrapper>, ContractError> {
    let mut messages: Vec<CosmosMsg<TerraMsgWrapper>> = vec![];

    let pair_addr = "terra156v8s539wtz0sjpn8y8a8lfg8fhmwa7fy22aff".to_string();

    let amount = Uint128::new(10);

    let swap_asset = Asset {
        info: AssetInfo::NativeToken {
            denom: "uusd".to_string(),
        },
        amount,
    };

    messages = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: pair_addr,
        msg: to_binary(&TerraswapExecuteMsg::Swap {
            offer_asset: Asset {
                amount,
                ..swap_asset
            },
            max_spread: None,
            belief_price: None,
            to: None,
        })?,
        funds: vec![Coin {
            denom: "uusd".to_string(),
            amount,
        }],
    })];

    Ok(Response::new()
        .add_attributes(vec![attr("action", "swap_msg")])
        .add_messages(messages))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}
