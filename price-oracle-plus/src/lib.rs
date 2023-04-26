#![no_std]

mod test;
mod extensions;

use shared::constants::Constants;
use shared::price_oracle::PriceOracle;
use shared::extensions::{env_extensions::EnvExtensions};
use shared::types::{error::Error, config_data::ConfigData, price_data::PriceData};
use extensions::env_balance_extensions::EnvBalanceExtensions;
use soroban_sdk::{contractimpl, panic_with_error, Address, BytesN, Env, Vec};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

pub struct PriceOracleContract;

#[contractimpl]
impl PriceOracleContract {
    //Admin section

    pub fn config(e: Env, user: Address, config: ConfigData) {
        let base_fee = config.base_fee;
        PriceOracle::config(&e, user, config);
        e.set_base_fee(base_fee);
    }

    pub fn add_assets(e: Env, user: Address, assets: Vec<Address>) {
        PriceOracle::add_assets(&e, user, assets)
    }

    pub fn set_fee(e: Env, user: Address, fee: i128) {
        e.panic_if_not_admin(&user);
        e.set_base_fee(fee);
    }

    pub fn set_price(e: Env, user: Address, updates: Vec<i128>, timestamp: u64) {
        PriceOracle::set_price(&e, user, updates, timestamp)
    }

    //end of admin section

    //Balance section

    pub fn deposit(e: Env, user: Address, account: BytesN<32>, asset: Address, amount: i128) {
        user.require_auth();
        if amount <= 0 {
            panic_with_error!(&e, Error::InvalidDepositAmount);
        }
        let fee_asset = fee_asset(&e);
        if fee_asset != asset {
            panic_with_error!(&e, Error::InvalidFeeAsset);
        }
        let token = token::Client::new(&e, &asset.contract_id().unwrap());
        token.xfer(&user, &e.current_contract_address(), &amount);
        e.try_inc_balance(account, amount);
    }

    pub fn balance(e: Env, account: BytesN<32>) -> Option<i128> {
        e.get_balance(account)
    }

    pub fn fee_asset(e: Env) -> Address {
        fee_asset(&e)
    }

    pub fn base_fee(e: Env) -> Option<i128> {
        e.get_base_fee()
    }

    //end of balance section

    pub fn admin(e: Env) -> Address {
        PriceOracle::admin(&e)
    }

    pub fn base(e: Env) -> Address {
        PriceOracle::base(&e)
    }

    pub fn decimals(e: Env) -> u32 {
        PriceOracle::decimals(&e)
    }

    pub fn resolution(e: Env) -> u32 {
        PriceOracle::resolution(&e)
    }

    pub fn period(e: Env) -> Option<u64> {
        PriceOracle::period(&e)
    }

    pub fn assets(e: Env) -> Option<Vec<Address>> {
        PriceOracle::assets(&e)
    }

    pub fn price(e: Env, asset: Address, timestamp: u64) -> Option<PriceData> {
        let invoker = get_invoker_or_panic(&e);
        charge_or_panic(&e, invoker, 1);
        let price = PriceOracle::price(&e, asset, timestamp);
        if price.is_none() {
            return None;
        }
        price
    }

    pub fn lastprice(e: Env, asset: Address) -> Option<PriceData> {
        let invoker = get_invoker_or_panic(&e);
        charge_or_panic(&e, invoker, 1);
        let price = PriceOracle::lastprice(&e, asset);
        if price.is_none() {
            return None;
        }
        price
    }

    pub fn x_price(
        e: Env,
        base_asset: Address,
        quote_asset: Address,
        timestamp: u64,
    ) -> Option<PriceData> {        
        let invoker = get_invoker_or_panic(&e);
        charge_or_panic(&e, invoker, 2);
        let price = PriceOracle::x_price(&e, base_asset, quote_asset, timestamp);
        if price.is_none() {
            return None;
        }
        price
    }

    pub fn x_lt_price(e: Env, base_asset: Address, quote_asset: Address) -> Option<PriceData> {
        let invoker = get_invoker_or_panic(&e);
        charge_or_panic(&e, invoker, 2);
        let price = PriceOracle::x_lt_price(&e, base_asset, quote_asset);
        if price.is_none() {
            return None;
        }
        price
    }

    pub fn prices(e: Env, asset: Address, records: u32) -> Option<Vec<PriceData>> {
        let invoker = get_invoker_or_panic(&e);
        charge_or_panic(&e, invoker, records); //TODO: check price multiplier
        let price =  PriceOracle::prices(&e, asset, records);
        if price.is_none() {
            return None;
        }
        price
    }

    pub fn x_prices(
        e: Env,
        base_asset: Address,
        quote_asset: Address,
        records: u32,
    ) -> Option<Vec<PriceData>> {
        let invoker = get_invoker_or_panic(&e);
        charge_or_panic(&e, invoker, records * 2);//TODO: check price multiplier
        let prices = PriceOracle::x_prices(&e, base_asset, quote_asset, records);
        if prices.is_none() {
            return None;
        }
        prices
    }

    pub fn twap(e: Env, asset: Address, records: u32) -> Option<i128> {
        let invoker = get_invoker_or_panic(&e);
        charge_or_panic(&e, invoker, records);
        let prices = PriceOracle::twap(&e, asset, records);
        if prices.is_none() {
            return None;
        }
        prices
    }

    pub fn x_twap(e: Env, base_asset: Address, quote_asset: Address, records: u32) -> Option<i128> {
        let invoker = get_invoker_or_panic(&e);
        charge_or_panic(&e, invoker, records);
        let prices = PriceOracle::x_twap(&e, base_asset, quote_asset, records);
        if prices.is_none() {
            return None;
        }
        prices
    }
}

fn fee_asset(e: &Env) -> Address {
    let bytes = BytesN::from_array(e, &Constants::FEE_ASSET);
    Address::from_contract_id(&e, &bytes)
}

fn get_invoker_or_panic(e: &Env) -> BytesN<32> {
    let invoker = e.invoker();
    if invoker.is_none() {
        panic_with_error!(e, Error::Unauthorized)
    }
    invoker.unwrap()
}

fn charge_or_panic(e: &Env, account: BytesN<32>, multiplier: u32) {
    let base_fee = e.get_base_fee().unwrap_or_else(||0);
    let amount = -(base_fee * multiplier as i128);
    if !e.try_inc_balance(account, amount) { 
        panic_with_error!(&e, Error::InsufficientBalance) 
    }
}