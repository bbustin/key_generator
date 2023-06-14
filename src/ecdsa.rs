use anyhow::Error;

use crate::{Algorithm, Base64EncodedKeys};

pub fn generate_keys(_algorithm: &Algorithm) -> Result<Base64EncodedKeys, Error> {
    Err(Error::msg("not implemented"))
}
