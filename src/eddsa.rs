use anyhow::Error;
use rand_core::{OsRng, RngCore};
use ring_compat::signature::ed25519::SigningKey;

use crate::{base64_encoder, Algorithm, Base64EncodedKeys};

pub fn generate_keys(_: &Algorithm) -> Result<Base64EncodedKeys, Error> {
    let mut ed25519_seed = [0u8; 32];
    OsRng.fill_bytes(&mut ed25519_seed);

    let signing_key = SigningKey::from_slice(&ed25519_seed)?;
    let verifying_key = signing_key.verifying_key();

    base64_encoder::encode(&signing_key.to_bytes(), verifying_key.as_ref())
}
