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
pub struct InlineResponse20015 {
    #[serde(rename = "storage")]
    pub storage: crate::models::Micheline007PsDelph1MichelsonV1Expression,
    #[serde(rename = "operations")]
    pub operations: Vec<crate::models::Model007PsDelph1OperationAlphaInternalOperation>,
    #[serde(rename = "big_map_diff", skip_serializing_if = "Option::is_none")]
    pub big_map_diff: Option<Vec<crate::models::OneOfobjectobjectobjectobject>>,
}

impl InlineResponse20015 {
    pub fn new(storage: crate::models::Micheline007PsDelph1MichelsonV1Expression, operations: Vec<crate::models::Model007PsDelph1OperationAlphaInternalOperation>) -> InlineResponse20015 {
        InlineResponse20015 {
            storage,
            operations,
            big_map_diff: None,
        }
    }
}

