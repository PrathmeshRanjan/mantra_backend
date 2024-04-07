use cosmwasm_std::{
    attr, entry_point, to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult, StdError, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg, Cw20Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CONTRACT_NAME: &str = "rwa-om-liquidity-pool";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub om_token_address: String, // Address of the OM token contract
    // Consider adding RWA token addresses if RWAs are tokenized
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    DepositOmToken { amount: Uint128 },
    DepositRwaToken { token_id: String, rwa_token_address: String, valuation: Uint128 },
    Withdraw { asset: Asset },
    Receive(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq)]
pub enum Asset {
    OmToken(Uint128),
    RwaToken { token_id: String, rwa_token_address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PoolInfo {
    pub total_om_tokens: Uint128,
    // Additional fields for RWA tracking if needed
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let pool_info = PoolInfo {
        total_om_tokens: Uint128::zero(),
        // Initialize fields for RWA
    };
    deps.storage.save(&pool_info)?;

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
        ExecuteMsg::DepositOmToken { amount } => deposit_om_token(deps, env, info, amount),
        ExecuteMsg::DepositRwaToken { token_id, rwa_token_address, valuation } => 
            deposit_rwa_token(deps, info, token_id, rwa_token_address, valuation),
        ExecuteMsg::Withdraw { asset } => withdraw_assets(deps, env, info, asset),
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
    }
}

fn deposit_om_token(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    amount: Uint128,
) -> Result<Response, StdError> {
    let mut pool_info: PoolInfo = deps.storage.load()?;
    pool_info.total_om_tokens += amount;
    deps.storage.save(&pool_info)?;

    Ok(Response::new()
        .add_attribute("action", "deposit_om_token")
        .add_attribute("amount", amount.to_string()))
}

fn deposit_rwa_token(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    rwa_token_address: String,
    _valuation: Uint128,
) -> Result<Response, StdError> {
    // RWA token deposit logic here
    // You would need a way to verify ownership and lock the token or represent the deposit in some way

    Ok(Response::new()
        .add_attribute("action", "deposit_rwa_token")
        .add_attribute("from", info.sender)
        .add_attribute("token_id", token_id)
        .add_attribute("rwa_token_address", rwa_token_address))
}

fn withdraw_assets(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    asset: Asset,
) -> Result<Response, StdError> {
    match asset {
        Asset::OmToken(amount) => {
            // Logic for withdrawing OM tokens
            let mut pool_info: PoolInfo = deps.storage.load()?;
            if amount > pool_info.total_om_tokens {
                return Err(StdError::generic_err("Not enough OM tokens in the pool"));
            }
            pool_info.total_om_tokens -= amount;
            deps.storage.save(&pool_info)?;

            // Transfer OM tokens back to the requester
            let om_transfer_msg = Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount,
            };

            let wasm_msg = WasmMsg::Execute {
                contract_addr: // OM Token Contract Address here,
                msg: to_binary(&om_transfer_msg)?,
                funds: vec![],
            };

            Ok(Response::new()
                .add_message(wasm_msg.into())
                .add_attribute("action", "withdraw_om_token")
                .add_attribute("amount", amount.to_string()))
        }
        Asset::RwaToken { token_id, rwa_token_address } => {
            // Logic for withdrawing RWA tokens
            // This would involve transferring the RWA token back to the owner and possibly updating internal state to reflect the withdrawal

            Ok(Response::new()
                .add_attribute("action", "withdraw_rwa_token")
                .add_attribute("token_id", token_id)
                .add_attribute("rwa_token_address", rwa_token_address))
        }
    }
}

fn receive_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, StdError> {
    // Handle CW20 tokens received
    // You could use this to handle receiving OM tokens for deposits

    Ok(Response::new()
        .add_attribute("action", "receive_cw20")
        .add_attribute("from", cw20_msg.sender)
        .add_attribute("amount", cw20_msg.amount.to_string()))
}
