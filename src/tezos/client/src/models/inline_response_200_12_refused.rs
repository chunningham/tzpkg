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
pub struct InlineResponse20012Refused {
    #[serde(rename = "hash")]
    pub hash: crate::models::OperationHash,
    #[serde(rename = "branch")]
    pub branch: crate::models::BlockHash,
    #[serde(rename = "data")]
    pub data: String,
    /// The full list of error is available with the global RPC `GET errors`
    #[serde(rename = "error")]
    pub error: Option<serde_json::Value>,
}

impl InlineResponse20012Refused {
    pub fn new(hash: crate::models::OperationHash, branch: crate::models::BlockHash, data: String, error: Option<serde_json::Value>) -> InlineResponse20012Refused {
        InlineResponse20012Refused {
            hash,
            branch,
            data,
            error,
        }
    }
}

