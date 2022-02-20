#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    attr, to_binary, Addr, Coin, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response,
    StdError, StdResult, Uint128,
};

use cw2::set_contract_version;
// use rand::seq::SliceRandom;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use terra_cosmwasm::{
    create_swap_msg, create_swap_send_msg, ContractInfoResponse, ExchangeRatesResponse,
    SwapResponse, TaxCapResponse, TaxRateResponse, TerraMsgWrapper, TerraQuerier,
};

const CONTRACT_NAME: &str = "crates.io:terra_swap_cases";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<TerraMsgWrapper>, StdError> {
    match msg {
        ExecuteMsg::MsgSwap {} => execute_single_msg_swap(deps, env, info),
        ExecuteMsg::MsgSwapSend {
            offer_coin,
            ask_denom,
            recipient,
        } => execute_msg_swap(deps, env, info, offer_coin, ask_denom, Some(recipient)),
        ExecuteMsg::MsgSwapUlunas { number_of_swapmsgs } => {
            execute_msg_swap_ulunas(deps, env, info, number_of_swapmsgs)
        }
        ExecuteMsg::MsgSwapRandom { number_of_swapmsgs } => {
            execute_msg_swap_random(deps, env, info, number_of_swapmsgs)
        }
        ExecuteMsg::MsgSwapSendUlunas {
            validator_address,
            number_of_swapmsgs,
        } => execute_msg_swap_send_ulunas(deps, env, info, validator_address, number_of_swapmsgs),
        ExecuteMsg::MsgSwapSendRamdom {
            validator_address,
            number_of_swapmsgs,
        } => execute_msg_swap_send_random(deps, env, info, validator_address, number_of_swapmsgs),
    }
}

pub fn execute_msg_swap_ulunas(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    number_of_swapmsgs: u64,
) -> StdResult<Response<TerraMsgWrapper>> {
    let mut messages = vec![];
    for _i in 0..number_of_swapmsgs {
        let curr_coin = Coin {
            amount: Uint128::new(1),
            denom: "uluna".to_string(),
        };
        messages.push(create_swap_msg(curr_coin, "uusd".to_string()));
    }
    let res = Response::new()
        .add_messages(messages)
        .add_attributes(vec![attr("action", "swap")]);
    Ok(res)
}

pub fn execute_msg_swap_random(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    number_of_swapmsgs: u64,
) -> StdResult<Response<TerraMsgWrapper>> {
    let mut messages = vec![];
    let denoms = vec![
        "uluna".to_string(),
        "usdr".to_string(),
        "uluna".to_string(),
        "ukrw".to_string(),
        "umnt".to_string(),
        "ueur".to_string(),
        "ucny".to_string(),
        "ujpy".to_string(),
        "ugbp".to_string(),
        "uinr".to_string(),
        "ucad".to_string(),
        "uchf".to_string(),
        "uaud".to_string(),
        "usgd".to_string(),
        "uthb".to_string(),
        "usek".to_string(),
        "unok".to_string(),
        "udkk".to_string(),
        "uidr".to_string(),
        "uphp".to_string(),
        "uhkd".to_string(),
    ];
    let mut curr_idx = 0;
    for _i in 0..number_of_swapmsgs {
        let curr_coin = Coin {
            amount: Uint128::new(1),
            denom: denoms[curr_idx].clone(),
        };
        messages.push(create_swap_msg(curr_coin, "uluna".to_string()));
        curr_idx += 1;
        if curr_idx == denoms.len() {
            curr_idx = 0;
        }
    }
    let res = Response::new()
        .add_messages(messages)
        .add_attributes(vec![attr("action", "swap")]);
    Ok(res)
}

pub fn execute_msg_swap_send_ulunas(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    validator_address: Addr,
    number_of_swapmsgs: u64,
) -> StdResult<Response<TerraMsgWrapper>> {
    let mut messages = vec![];
    for _i in 0..number_of_swapmsgs {
        let curr_coin = Coin {
            amount: Uint128::new(1),
            denom: "uluna".to_string(),
        };
        messages.push(create_swap_send_msg(
            validator_address.to_string(),
            curr_coin,
            "uusd".to_string(),
        ));
    }
    let res = Response::new()
        .add_messages(messages)
        .add_attributes(vec![attr("action", "swap")]);
    Ok(res)
}

pub fn execute_msg_swap_send_random(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    validator_address: Addr,
    number_of_swapmsgs: u64,
) -> StdResult<Response<TerraMsgWrapper>> {
    let mut messages = vec![];
    let denoms = vec![
        "uluna".to_string(),
        "usdr".to_string(),
        "uluna".to_string(),
        "ukrw".to_string(),
        "umnt".to_string(),
        "ueur".to_string(),
        "ucny".to_string(),
        "ujpy".to_string(),
        "ugbp".to_string(),
        "uinr".to_string(),
        "ucad".to_string(),
        "uchf".to_string(),
        "uaud".to_string(),
        "usgd".to_string(),
        "uthb".to_string(),
        "usek".to_string(),
        "unok".to_string(),
        "udkk".to_string(),
        "uidr".to_string(),
        "uphp".to_string(),
        "uhkd".to_string(),
    ];

    let mut curr_idx = 0;

    for _i in 0..number_of_swapmsgs {
        let curr_coin = Coin {
            amount: Uint128::new(1),
            denom: denoms[curr_idx].clone(),
        };
        messages.push(create_swap_send_msg(
            validator_address.to_string(),
            curr_coin,
            "uluna".to_string(),
        ));
        curr_idx += 1;
        if curr_idx == denoms.len() {
            curr_idx = 0;
        }
    }
    let res = Response::new()
        .add_messages(messages)
        .add_attributes(vec![attr("action", "swap")]);
    Ok(res)
}

pub fn execute_single_msg_swap(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> StdResult<Response<TerraMsgWrapper>> {
    let curr_coin = Coin {
        amount: Uint128::new(1),
        denom: "uusd".to_string(),
    };
    let msg = create_swap_msg(curr_coin, "uluna".to_string());

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_swap(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    offer_coin: Coin,
    ask_denom: String,
    recipient: Option<Addr>,
) -> StdResult<Response<TerraMsgWrapper>> {
    let msg = if let Some(recipient) = recipient {
        create_swap_send_msg(recipient.to_string(), offer_coin, ask_denom)
    } else {
        create_swap_msg(offer_coin, ask_denom)
    };

    Ok(Response::new().add_message(msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Swap {
            offer_coin,
            ask_denom,
        } => to_binary(&query_swap(deps, offer_coin, ask_denom)?),
        QueryMsg::TaxRate {} => to_binary(&query_tax_rate(deps)?),
        QueryMsg::TaxCap { denom } => to_binary(&query_tax_cap(deps, denom)?),
        QueryMsg::ExchangeRates {
            base_denom,
            quote_denoms,
        } => to_binary(&query_exchange_rates(deps, base_denom, quote_denoms)?),
        QueryMsg::ContractInfo { contract_address } => {
            to_binary(&query_contract_info(deps, contract_address)?)
        }
    }
}

pub fn query_swap(deps: Deps, offer_coin: Coin, ask_denom: String) -> StdResult<SwapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: SwapResponse = querier.query_swap(offer_coin, ask_denom)?;

    Ok(res)
}

pub fn query_tax_rate(deps: Deps) -> StdResult<TaxRateResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: TaxRateResponse = querier.query_tax_rate()?;

    Ok(res)
}

pub fn query_tax_cap(deps: Deps, denom: String) -> StdResult<TaxCapResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: TaxCapResponse = querier.query_tax_cap(denom)?;

    Ok(res)
}

pub fn query_exchange_rates(
    deps: Deps,
    base_denom: String,
    quote_denoms: Vec<String>,
) -> StdResult<ExchangeRatesResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: ExchangeRatesResponse = querier.query_exchange_rates(base_denom, quote_denoms)?;

    Ok(res)
}

pub fn query_contract_info(
    deps: Deps,
    contract_address: String,
) -> StdResult<ContractInfoResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: ContractInfoResponse = querier.query_contract_info(contract_address)?;

    Ok(res)
}
