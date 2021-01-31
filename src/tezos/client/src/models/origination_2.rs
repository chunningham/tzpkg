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
pub struct Origination2 {
    #[serde(rename = "kind")]
    pub kind: Kind,
    #[serde(rename = "source")]
    pub source: crate::models::SignaturePublicKeyHash,
    /// Decimal representation of a positive big number
    #[serde(rename = "fee")]
    pub fee: String,
    /// Decimal representation of a positive big number
    #[serde(rename = "counter")]
    pub counter: String,
    /// Decimal representation of a positive big number
    #[serde(rename = "gas_limit")]
    pub gas_limit: String,
    /// Decimal representation of a positive big number
    #[serde(rename = "storage_limit")]
    pub storage_limit: String,
    /// Decimal representation of a positive big number
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "delegate", skip_serializing_if = "Option::is_none")]
    pub delegate: Option<crate::models::SignaturePublicKeyHash>,
    #[serde(rename = "script")]
    pub script: crate::models::Model007PsDelph1ScriptedContracts,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::Origination2Metadata,
}

impl Origination2 {
    pub fn new(kind: Kind, source: crate::models::SignaturePublicKeyHash, fee: String, counter: String, gas_limit: String, storage_limit: String, balance: String, script: crate::models::Model007PsDelph1ScriptedContracts, metadata: crate::models::Origination2Metadata) -> Origination2 {
        Origination2 {
            kind,
            source,
            fee,
            counter,
            gas_limit,
            storage_limit,
            balance,
            delegate: None,
            script,
            metadata,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "origination")]
    Origination,
}
