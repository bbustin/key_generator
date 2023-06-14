use anyhow::Error;
use base64::Engine as _;

use crate::Base64EncodedKeys;

pub fn encode(private_key: &[u8], public_key: &[u8]) -> Result<Base64EncodedKeys, Error> {
    let encoded_keys = Base64EncodedKeys {
        private_key: base64::engine::general_purpose::STANDARD.encode(private_key),
        public_key: base64::engine::general_purpose::STANDARD.encode(public_key),
    };

    Ok(encoded_keys)
}
