mod base64_encoder;
mod ecdsa;
mod eddsa;
mod rsa;

use anyhow::Error;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone)]
pub enum Algorithm {
    RS256,
    RS384,
    RS512,
    PS256,
    PS384,
    PS512,
    ES256,
    ES384,
    EDDSA,
}

#[derive(Debug)]
pub struct Base64EncodedKeys {
    private_key: String,
    public_key: String,
}

trait EncodedKeyGenerator {
    fn generate_keys(algorithm: Algorithm) -> Result<Base64EncodedKeys, Error>;
}

/// Program to generate Base64 encoded public/private keys
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// JWT algorithm
    #[arg(value_enum)]
    algorithm: Algorithm,
}

fn main() {
    let args = Args::parse();

    let algorithm = args.algorithm;

    let access_token = get_keypair(&algorithm).expect("unable to generate access token key pair");
    let refresh_token = get_keypair(&algorithm).expect("unable to generate refresh token key pair");

    println!(
        "TOKEN_ALGORITHM={}

ACCESS_TOKEN_PRIVATE_KEY={}
ACCESS_TOKEN_PUBLIC_KEY={}
ACCESS_TOKEN_EXPIRED_IN=15m
ACCESS_TOKEN_MAXAGE=15

REFRESH_TOKEN_PRIVATE_KEY={}
REFRESH_TOKEN_PUBLIC_KEY={}
REFRESH_TOKEN_EXPIRED_IN=60m
REFRESH_TOKEN_MAXAGE=60",
        algorithm
            .to_possible_value()
            .expect("can not get algorithm value")
            .get_name(),
        access_token.private_key,
        access_token.public_key,
        refresh_token.private_key,
        refresh_token.public_key
    );
}

fn get_keypair(algorithm: &Algorithm) -> Result<Base64EncodedKeys, Error> {
    match algorithm {
        Algorithm::RS256
        | Algorithm::RS384
        | Algorithm::RS512
        | Algorithm::PS256
        | Algorithm::PS384
        | Algorithm::PS512 => rsa::generate_keys(algorithm),

        Algorithm::ES256 | Algorithm::ES384 => ecdsa::generate_keys(algorithm),

        Algorithm::EDDSA => eddsa::generate_keys(algorithm),
    }
}
