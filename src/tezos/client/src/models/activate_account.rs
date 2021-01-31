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
pub struct ActivateAccount {
    #[serde(rename = "kind")]
    pub kind: Kind,
    #[serde(rename = "pkh")]
    pub pkh: crate::models::Ed25519PublicKeyHash,
    #[serde(rename = "secret")]
    pub secret: String,
}

impl ActivateAccount {
    pub fn new(kind: Kind, pkh: crate::models::Ed25519PublicKeyHash, secret: String) -> ActivateAccount {
        ActivateAccount {
            kind,
            pkh,
            secret,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "activate_account")]
    ActivateAccount,
}
