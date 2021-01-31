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
pub struct Applied {
    #[serde(rename = "status")]
    pub status: Status,
    /// Decimal representation of a positive big number
    #[serde(rename = "consumed_gas", skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    /// Decimal representation of a positive big number
    #[serde(rename = "consumed_milligas", skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
}

impl Applied {
    pub fn new(status: Status) -> Applied {
        Applied {
            status,
            consumed_gas: None,
            consumed_milligas: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "applied")]
    Applied,
}
