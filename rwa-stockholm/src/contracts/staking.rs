// This contract will allow users to stake their RWAs represented as NFTs (assuming a CW721-compatible standard) 
// and earn "OM" tokens (assuming a CW20-compatible standard) over time based on the staking period and the 
// value of the staked asset.

use cosmwasm_std::{
    attr, entry_point, to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg, from_binary,
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg, Cw20Coin};
use cw721::{Cw721ReceiveMsg, NftInfoResponse, OwnerOfResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const CONTRACT_NAME: &str = "rwa-staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub om_token_address: String, // Address of the OM token contract
    pub reward_rate_per_day: Uint128, // Base reward rate per day for staking
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    StakeNft { nft_contract_address: String, token_id: String },
    UnstakeNft { token_id: String },
    ClaimRewards { token_id: String },
    ReceiveNft(Cw721ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakedAsset {
    pub owner: String,
    pub nft_contract_address: String,
    pub token_id: String,
    pub staked_since: u64, // Unix timestamp
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingInfo {
    pub staked_assets: HashMap<String, StakedAsset>, // Keyed by token_id
    pub total_staked: u64,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let staking_info = StakingInfo {
        staked_assets: HashMap::new(),
        total_staked: 0,
    };
    deps.storage.save(&staking_info)?;

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
        ExecuteMsg::StakeNft { nft_contract_address, token_id } => {
            stake_nft(deps, env, info, nft_contract_address, token_id)
        }
        ExecuteMsg::UnstakeNft { token_id } => {
            unstake_nft(deps, env, info, token_id)
        }
        ExecuteMsg::ClaimRewards { token_id } => {
            claim_rewards(deps, env, info, token_id)
        }
        ExecuteMsg::ReceiveNft(msg) => receive_nft(deps, env, info, msg),
    }
}

fn stake_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    nft_contract_address: String,
    token_id: String,
) -> Result<Response, StdError> {
    // Verify ownership, stake logic here
    // For simplicity, assuming direct call without CW721 ReceiveMsg

    let staker = info.sender;
    let staked_asset = StakedAsset {
        owner: staker.to_string(),
        nft_contract_address: nft_contract_address.clone(),
        token_id: token_id.clone(),
        staked_since: env.block.time.seconds(),
    };

    let mut staking_info: StakingInfo = deps.storage.load()?;
    staking_info.staked_assets.insert(token_id.clone(), staked_asset);
    deps.storage.save(&staking_info)?;

    Ok(Response::new()
        .add_attribute("action", "stake_nft")
        .add_attribute("nft_contract_address", nft_contract_address)
        .add_attribute("token_id", token_id)
        .add_attribute("staker", staker))
}

fn unstake_nft(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, StdError> {
    let mut staking_info: StakingInfo = deps.storage.load()?;
    let staked_asset = staking_info.staked_assets.get(&token_id).ok_or(StdError::generic_err("NFT not staked"))?;

    if staked_asset.owner != info.sender.to_string() {
        return Err(StdError::unauthorized());
    }

    staking_info.staked_assets.remove(&token_id);
    deps.storage.save(&staking_info)?;

    Ok(Response::new()
        .add_attribute("action", "unstake_nft")
        .add_attribute("token_id", token_id))
}

fn claim_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, StdError> {
    let staking_info: StakingInfo = deps.storage.load()?;
    let staked_asset = staking_info.staked_assets.get(&token_id).ok_or(StdError::generic_err("NFT not staked"))?;

    if staked_asset.owner != info.sender.to_string() {
        return Err(StdError::unauthorized());
    }

    // Calculate rewards based on time staked
    // This is a simplified calculation; real-world usage might consider asset value, dynamic rates, etc.
    let reward_rate_per_day: Uint128 = // Load from contract state
    let time_staked = env.block.time.seconds() - staked_asset.staked_since;
    let days_staked = time_staked / 86400; // Seconds in a day
    let rewards = reward_rate_per_day * Uint128::from(days_staked);

    // Transfer OM tokens as rewards
    let om_transfer_msg = Cw20ExecuteMsg::Transfer {
        recipient: info.sender.to_string(),
        amount: rewards,
    };

    let wasm_msg = WasmMsg::Execute {
        contract_addr: // OM Token Contract Address here,
        msg: to_binary(&om_transfer_msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(wasm_msg.into())
        .add_attribute("action", "claim_rewards")
        .add_attribute("token_id", token_id)
        .add_attribute("rewards", rewards.to_string()))
}

fn receive_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw721ReceiveMsg,
) -> Result<Response, StdError> {
    //
