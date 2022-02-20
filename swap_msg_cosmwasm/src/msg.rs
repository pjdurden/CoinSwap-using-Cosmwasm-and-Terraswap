use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    MsgSwap {},
    MsgSwapSend {
        offer_coin: Coin,
        ask_denom: String,
        recipient: Addr,
    },
    MsgSwapUlunas {
        number_of_swapmsgs: u64,
    },
    MsgSwapSendUlunas {
        validator_address: Addr,
        number_of_swapmsgs: u64,
    },
    MsgSwapRandom {
        number_of_swapmsgs: u64,
    },
    MsgSwapSendRamdom {
        validator_address: Addr,
        number_of_swapmsgs: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Swap {
        offer_coin: Coin,
        ask_denom: String,
    },
    TaxRate {},
    TaxCap {
        denom: String,
    },
    ExchangeRates {
        base_denom: String,
        quote_denoms: Vec<String>,
    },
    ContractInfo {
        contract_address: String,
    },
}
