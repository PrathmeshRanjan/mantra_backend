// use cosmwasm_schema::{cw_serde, QueryResponses};

// #[cw_serde]
// pub struct InstantiateMsg {}

// #[cw_serde]
// pub enum ExecuteMsg {}

// #[cw_serde]
// #[derive(QueryResponses)]
// pub enum QueryMsg {}
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}