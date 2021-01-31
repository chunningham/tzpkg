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
pub struct InlineResponse20018 {
    #[serde(rename = "gas")]
    pub gas: crate::models::OneOfobjectstring,
}

impl InlineResponse20018 {
    pub fn new(gas: crate::models::OneOfobjectstring) -> InlineResponse20018 {
        InlineResponse20018 {
            gas,
        }
    }
}

