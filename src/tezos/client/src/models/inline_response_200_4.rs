/*
 * Tezos RPC
 *
 * Tezos client RPC API.
 *
 * The version of the OpenAPI document: 7.5
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2004 {
    /// Decimal representation of a positive big number
    #[serde(rename = "balance")]
    pub balance: String,
    /// Decimal representation of a positive big number
    #[serde(rename = "frozen_balance")]
    pub frozen_balance: String,
    #[serde(rename = "frozen_balance_by_cycle")]
    pub frozen_balance_by_cycle: Vec<crate::models::InlineResponse2004FrozenBalanceByCycle>,
    /// Decimal representation of a positive big number
    #[serde(rename = "staking_balance")]
    pub staking_balance: String,
    #[serde(rename = "delegated_contracts")]
    pub delegated_contracts: Vec<crate::models::Model007PsDelph1ContractId>,
    /// Decimal representation of a positive big number
    #[serde(rename = "delegated_balance")]
    pub delegated_balance: String,
    #[serde(rename = "deactivated")]
    pub deactivated: bool,
    #[serde(rename = "grace_period")]
    pub grace_period: i32,
}

impl InlineResponse2004 {
    pub fn new(balance: String, frozen_balance: String, frozen_balance_by_cycle: Vec<crate::models::InlineResponse2004FrozenBalanceByCycle>, staking_balance: String, delegated_contracts: Vec<crate::models::Model007PsDelph1ContractId>, delegated_balance: String, deactivated: bool, grace_period: i32) -> InlineResponse2004 {
        InlineResponse2004 {
            balance,
            frozen_balance,
            frozen_balance_by_cycle,
            staking_balance,
            delegated_contracts,
            delegated_balance,
            deactivated,
            grace_period,
        }
    }
}

