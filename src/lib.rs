#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

mod test;

use soroban_auth::{
    check_auth, NonceAuth, {Identifier, Signature},
};
use soroban_sdk::{contractimpl, contracttype, symbol, BigInt, BytesN, Env, Vec, IntoVal, vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetName {
    pub name: BytesN<16>,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Nonce(Identifier),
    Asset(AssetName),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssetPrice {
    None,
    AssetPrice(AssetPriceData),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetPriceData {
    pub price: u64,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetPriceUpdate {
    pub asset: AssetName,
    pub price: u64,
}

fn read_nonce(e: &Env, id: Identifier) -> BigInt {
    let key = DataKey::Nonce(id);
    e.contract_data()
        .get(key)
        .unwrap_or_else(|| Ok(BigInt::zero(e)))
        .unwrap()
}

struct NonceForSignature(Signature);

impl NonceAuth for NonceForSignature {
    fn read_nonce(e: &Env, id: Identifier) -> BigInt {
        read_nonce(e, id)
    }

    fn read_and_increment_nonce(&self, e: &Env, id: Identifier) -> BigInt {
        let key = DataKey::Nonce(id.clone());
        let nonce = Self::read_nonce(e, id);
        e.contract_data().set(key, &nonce + 1);
        nonce
    }

    fn signature(&self) -> &Signature {
        &self.0
    }
}

pub struct OracleContract;

#[contractimpl]
impl OracleContract {

    fn is_initialized(e: &Env) -> bool {
        e.contract_data().has(DataKey::Admin)
    }

    //Set the admin identifier.
    pub fn init(e: Env, admin: Identifier) {
        if Self::is_initialized(&e) {
            panic!("Contract already initialized");
        }
        e.contract_data().set(DataKey::Admin, admin);
    }

    //Get current admin nonce.
    pub fn nonce(e: Env, id: Identifier) -> BigInt {
        read_nonce(&e, id)
    }

    // Set prices for assets. Only admin can call this method.
    pub fn set_price(e: Env, sig: Signature, nonce: BigInt, updates: Vec<AssetPriceUpdate>) {
        if !Self::is_initialized(&e) {
            panic!("Not authorized by admin");
        }
        let auth_id = sig.get_identifier(&e);
        if auth_id != e.contract_data()
            .get_unchecked(DataKey::Admin)
            .unwrap() {
            panic!("Not authorized by admin")
        }

        check_auth(
            &e,
            &NonceForSignature(sig),
            nonce.clone(),
            symbol!("set_price"),
            vec![&e, auth_id.into_val(&e), nonce.into_val(&e), updates.clone().into_val(&e)],
        );

        //iterate over the updates
        for u in updates.iter() {
            if !u.is_ok() {
                //TODO: log error
                continue;
            }
            let update = u.ok().unwrap();

            //store the new price
            e.contract_data().set(
                &DataKey::Asset(update.asset),
                AssetPrice::AssetPrice(AssetPriceData {
                    price: update.price,
                    timestamp: e.ledger().timestamp(),
                }),
            );
        }
    }

    //Get the price for an asset.
    pub fn get_price(e: Env, asset: AssetName) -> AssetPrice {
        //get the current price
        let data = e.contract_data();
        let key = DataKey::Asset(asset);
        if !data.has(&key) {
            return AssetPrice::None;
        }
        let price_option = data.get_unchecked(&key);
        let price = price_option.unwrap();

        price
    }
}