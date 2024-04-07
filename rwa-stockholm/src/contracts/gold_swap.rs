/*
This contract provides a simplified version of a gold-to-"OM" token swap mechanism. It assumes that both gold and "OM" tokens are represented as CW20 tokens on the CosmWasm platform. The contract includes functionalities for setting the exchange rate by an admin and for users to swap their gold tokens for "OM" tokens based on the current rate.
*/

use cosmwasm_std::{
    attr, entry_point, to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, WasmMsg, Uint128,
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CONTRACT_NAME: &str = "gold-om-swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
    pub gold_token_address: String,
    pub om_token_address: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetExchangeRate { gold_to_om_rate: Uint128 },
    Receive(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    SwapGoldForOm {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub admin: String,
    pub gold_to_om_rate: Uint128,
    pub gold_token_address: String,
    pub om_token_address: String,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State {
        admin: msg.admin,
        gold_to_om_rate: Uint128::zero(), // Initialize with zero, expecting the admin to set the rate
        gold_token_address: msg.gold_token_address,
        om_token_address: msg.om_token_address,
    };
    deps.storage.save(&state)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::SetExchangeRate { gold_to_om_rate } => {
            execute_set_exchange_rate(deps, info, gold_to_om_rate)
        }
        ExecuteMsg::Receive(msg) => execute_receive(deps, env, info, msg),
    }
}

pub fn execute_set_exchange_rate(
    deps: DepsMut,
    info: MessageInfo,
    gold_to_om_rate: Uint128,
) -> Result<Response, StdError> {
    let mut state: State = deps.storage.load()?;
    if info.sender.to_string() != state.admin {
        return Err(StdError::unauthorized());
    }

    state.gold_to_om_rate = gold_to_om_rate;
    deps.storage.save(&state)?;

    Ok(Response::new()
        .add_attribute("action", "set_exchange_rate")
        .add_attribute("rate", gold_to_om_rate.to_string()))
}

pub fn execute_receive(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, StdError> {
    let state: State = deps.storage.load()?;
    if info.sender != state.gold_token_address {
        return Err(StdError::generic_err("This token is not allowed for swap"));
    }

    let receive_msg: ReceiveMsg = serde_json_wasm::from_slice(&cw20_msg.msg)?;
    match receive_msg {
        ReceiveMsg::SwapGoldForOm {} => {
            let om_amount = state.gold_to_om_rate.multiply_ratio(cw20_msg.amount, Uint128::from(1u128));
            let send_om_msg = WasmMsg::Execute {
                contract_addr: state.om_token_address.clone(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: cw20_msg.sender,
                    amount: om_amount,
                })?,
                funds: vec![],
            };

            Ok(Response::new()
                .add_message(CosmosMsg::Wasm(send_om_msg))
                .add_attribute("action", "swap_gold_for_om")
                .add_attribute("gold_amount", cw20_msg.amount.to_string())
                .add_attribute("om_amount", om_amount.to_string()))
        }
    }
}
