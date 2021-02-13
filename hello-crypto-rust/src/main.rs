use rand::{rngs::StdRng};
use rand_core::SeedableRng;
use hello_crypto_rust::ethereum::{EthereumPrivateKey, EthereumFormat};
use hello_crypto_rust::model::{PrivateKey, PrivateKeyError, AddressError, PublicKeyError, PublicKey};
// use std::str::FromStr;

#[macro_use] extern crate log;

fn main(){
    pretty_env_logger::init(); 
    new();
}
pub fn new() -> Result<EthereumPrivateKey, CreateError> {
    // let str_a = "8279d7c0ae2c3266b557845d50ede43e22a7e60408b7c90ee279b8848dbac771";
    // let private_key = EthereumPrivateKey::from_str(&str_a);
    let rng = &mut StdRng::from_entropy();
    let private_key = EthereumPrivateKey::new(rng)?;
    info!("priv: {}", private_key.to_string());
    let public_key = private_key.to_public_key();
    info!("pub: {}", public_key.to_string());
    let address = public_key.to_address(&EthereumFormat::Standard)?;
    info!("addr: {}", address.to_string());
    Ok(private_key)
}

pub enum CreateError {
    AddressError(AddressError),
    PrivateKeyError(PrivateKeyError),
    PublicKeyError(PublicKeyError)
}

impl From<AddressError> for CreateError {
    fn from(error: AddressError) -> Self {
        CreateError::AddressError(error)
    }
}

impl From<PrivateKeyError> for CreateError {
    fn from(error: PrivateKeyError) -> Self {
        CreateError::PrivateKeyError(error)
    }
}

impl From<PublicKeyError> for CreateError {
    fn from(error: PublicKeyError) -> Self {
        CreateError::PublicKeyError(error)
    }
}