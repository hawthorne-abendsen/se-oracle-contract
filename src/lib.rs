#![no_std]

mod test;

use soroban_auth::Identifier;
use soroban_sdk::{
    contracterror, contractimpl, contracttype, panic_error, Address, BigInt, Env, Vec,
};

#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    IncorrectNonce = 1,
    Unauthorized = 2,
    InvalidAddressType = 3,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Nonce(Address),
    Asset(Identifier),
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
#[derive(Clone, PartialEq)]
pub struct AssetPriceUpdate {
    pub asset: Identifier,
    pub price: u64,
}

fn verify_and_consume_nonce(env: &Env, invoker: &Address, nonce: &BigInt) {
    if nonce != &get_nonce(env, &invoker) {
        panic_error!(env, Error::IncorrectNonce);
    }
    set_nonce(env, invoker, nonce + 1);
}

fn get_nonce(env: &Env, id: &Address) -> BigInt {
    let key = DataKey::Nonce(id.clone());
    env.data()
        .get(key)
        .unwrap_or_else(|| Ok(BigInt::zero(env)))
        .unwrap()
}

fn set_nonce(env: &Env, id: &Address, nonce: BigInt) {
    let key = DataKey::Nonce(id.clone());
    env.data().set(key, nonce);
}

fn set_admin(e: &Env, admin: &Address) {
    match admin {
        Address::Account(_) => {}
        Address::Contract(_) => panic_error!(&e, Error::InvalidAddressType),
    }
    e.data().set(DataKey::Admin, admin);
}

fn is_authorized(e: &Env, invoker: &Address) -> bool {
    return invoker == &get_admin(&e);
}

fn check_authorization(e: &Env, invoker: &Address) {
    if !is_authorized(&e, &invoker) {
        panic_error!(&e, Error::Unauthorized);
    }
}

fn is_initialized(e: &Env) -> bool {
    e.data().has(DataKey::Admin)
}

fn get_admin(e: &Env) -> Address {
    return e.data().get_unchecked(DataKey::Admin).unwrap();
}

pub struct OracleContract;

#[contractimpl]
impl OracleContract {
    //Set the admin identifier.
    pub fn set_admin(e: Env, admin: Address) {
        let invoker = e.invoker();
        if is_initialized(&e) && !is_authorized(&e, &invoker) {
            panic_error!(&e, Error::Unauthorized);
        }
        set_admin(&e, &admin);
    }

    pub fn get_admin(e: Env) -> Option<Address> {
        if !is_initialized(&e) {
            return None;
        }
        Some(get_admin(&e))
    }

    //Get current admin nonce.
    pub fn nonce(e: Env) -> BigInt {
        let invoker = e.invoker();
        get_nonce(&e, &invoker)
    }

    // Set prices for assets. Only admin can call this method.
    pub fn set_price(e: Env, nonce: BigInt, updates: Vec<AssetPriceUpdate>) {
        if !is_initialized(&e) {
            panic_error!(&e, Error::Unauthorized);
        }

        let invoker = e.invoker();
        check_authorization(&e, &invoker);

        verify_and_consume_nonce(&e, &invoker, &nonce);

        //iterate over the updates
        for u in updates.iter() {
            if !u.is_ok() {
                //TODO: log error
                continue;
            }
            let update = u.ok().unwrap();

            //store the new price
            e.data().set(
                &DataKey::Asset(update.asset),
                AssetPrice::AssetPrice(AssetPriceData {
                    price: update.price,
                    timestamp: e.ledger().timestamp(),
                }),
            );
        }
    }

    //Get the price for an asset.
    pub fn get_price(e: Env, asset: Identifier) -> AssetPrice {
        //get the current price
        let data = e.data();
        let key = DataKey::Asset(asset);
        if !data.has(&key) {
            return AssetPrice::None;
        }
        let price_option = data.get_unchecked(&key);
        let price = price_option.unwrap();

        price
    }
}
