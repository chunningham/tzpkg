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
pub struct InlineResponse20012Operations {
    #[serde(rename = "applied")]
    pub applied: Vec<crate::models::InlineResponse20012Applied>,
    #[serde(rename = "refused")]
    pub refused: Vec<crate::models::InlineResponse20012Refused>,
    #[serde(rename = "branch_refused")]
    pub branch_refused: Vec<crate::models::InlineResponse20012Refused>,
    #[serde(rename = "branch_delayed")]
    pub branch_delayed: Vec<crate::models::InlineResponse20012Refused>,
}

impl InlineResponse20012Operations {
    pub fn new(applied: Vec<crate::models::InlineResponse20012Applied>, refused: Vec<crate::models::InlineResponse20012Refused>, branch_refused: Vec<crate::models::InlineResponse20012Refused>, branch_delayed: Vec<crate::models::InlineResponse20012Refused>) -> InlineResponse20012Operations {
        InlineResponse20012Operations {
            applied,
            refused,
            branch_refused,
            branch_delayed,
        }
    }
}

