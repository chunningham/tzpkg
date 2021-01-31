/*
 * Tezos RPC
 *
 * Tezos client RPC API.
 *
 * The version of the OpenAPI document: 7.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Unistring : Either a plain UTF8 string, or a sequence of bytes for strings that contain invalid byte sequences.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unistring {
    #[serde(rename = "invalid_utf8_string")]
    pub invalid_utf8_string: Vec<i32>,
}

impl Unistring {
    /// Either a plain UTF8 string, or a sequence of bytes for strings that contain invalid byte sequences.
    pub fn new(invalid_utf8_string: Vec<i32>) -> Unistring {
        Unistring {
            invalid_utf8_string,
        }
    }
}

