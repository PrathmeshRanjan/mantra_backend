/*
Key Components and Considerations:

1. Ownership Verification: Before listing an NFT for sale, the contract verifies that the caller (info.sender) is the current owner of the NFT. This ensures that only the rightful owner can initiate a sale.
2. Sale Information: When an NFT is listed for sale, the contract records the sale information, including the token ID, seller's address, and the sale price. This information is crucial for facilitating the purchase transaction later on.
3. Purchase Transaction: In the purchase function (try_buy_nft), the contract checks if the token ID matches an active listing and if the buyer has provided sufficient funds in the specified denomination (info.funds). Upon successful validation, the contract removes the sale listing, transfers the NFT to the buyer, and the sale funds to the seller.
4. Error Handling: The contract includes basic error handling, such as Cw721ContractError::Unauthorized for unauthorized actions and Cw721ContractError::InsufficientFunds for insufficient purchase funds. Robust error handling is critical for a production-ready contract.
5. Storage Management: The contract utilizes a simple storage mechanism (sales_storage) to record active sale listings. Depending on the scale and requirements, a more sophisticated storage solution might be necessary, especially to handle multiple active listings efficiently.
*/

use cosmwasm_std::{
    attr, entry_point, to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Storage, Uint128, WasmMsg, CosmosMsg,
};
use cw2::set_contract_version;
use cw721_base::{
    contract::{execute_mint, execute_transfer_nft, instantiate, query_owner_of},
    msg::{ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg, MintMsg},
    ContractError as Cw721ContractError, MinterResponse, NftInfoResponse, OwnerOfResponse,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CONTRACT_NAME: &str = "crates.io:rwa-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub cw721_base_msg: Cw721InstantiateMsg,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Cw721Base(Cw721ExecuteMsg),
    ListNftForSale { token_id: String, price: Coin },
    BuyNft { token_id: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SaleInfo {
    pub token_id: String,
    pub seller: String,
    pub price: Coin,
}

// Storage for sales
pub fn sales_storage(storage: &mut dyn Storage) -> Singleton<SaleInfo> {
    singleton(storage, b"sales_info")
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    instantiate(deps, env, info, msg.cw721_base_msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, Cw721ContractError> {
    match msg {
        ExecuteMsg::Cw721Base(base_msg) => match base_msg {
            Cw721ExecuteMsg::Mint(mint_msg) => execute_mint(deps, env, info, mint_msg),
            Cw721ExecuteMsg::TransferNft {
                recipient, token_id, ..
            } => execute_transfer_nft(deps, env, info, recipient, token_id),
            _ => Err(Cw721ContractError::Unauthorized {}),
        },
        ExecuteMsg::ListNftForSale { token_id, price } => {
            try_list_for_sale(deps, info, token_id, price)
        }
        ExecuteMsg::BuyNft { token_id } => try_buy_nft(deps, env, info, token_id),
    }
}

fn try_list_for_sale(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    price: Coin,
) -> Result<Response, Cw721ContractError> {
    let owner_of: OwnerOfResponse = query_owner_of(deps.as_ref(), env.clone(), token_id.clone())?;

    if info.sender != owner_of.owner {
        return Err(Cw721ContractError::Unauthorized {});
    }

    let sale_info = SaleInfo {
        token_id: token_id.clone(),
        seller: info.sender.to_string(),
        price,
    };

    sales_storage(deps.storage).save(&sale_info)?;

    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "list_for_sale"),
            attr("token_id", token_id),
            attr("seller", info.sender),
            attr("price", sale_info.price.to_string()),
        ]))
}

fn try_buy_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, Cw721ContractError> {
    let sale_info: SaleInfo = sales_storage(deps.storage).load()?;

    if sale_info.token_id != token_id {
        return Err(Cw721ContractError::Unauthorized {});
    }

    if info.funds.iter().any(|coin| coin.denom == sale_info.price.denom && coin.amount >= sale_info.price.amount) {
        sales_storage(deps.storage).remove();

        // Transfer the NFT to the buyer
        execute_transfer_nft(deps, env, info.clone(), info.sender.to_string(), token_id.clone())?;

        // Transfer funds to the seller
        let seller = deps.api.addr_validate(&sale_info.seller)?;
        let send_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: seller.to_string(),
            amount: vec![sale_info.price.clone()],
        });

        Ok(Response::new()
            .add_message(send_msg)
            .add_attributes(vec![
                attr("action", "buy_nft"),
                attr("token_id", token_id),
                attr("buyer", info.sender),
                attr("price", sale_info.price.to_string()),
            ]))
    } else {
        Err(Cw721ContractError::InsufficientFunds {})
    }
}
