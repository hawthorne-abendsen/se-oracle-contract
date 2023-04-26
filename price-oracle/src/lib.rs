#![no_std]

mod test;

use shared::price_oracle::PriceOracle;
use shared::types::{config_data::ConfigData, price_data::PriceData};
use soroban_sdk::{contractimpl, Address, Env, Vec};
pub struct PriceOracleContract;

#[contractimpl]
impl PriceOracleContract {
    //Admin section

    pub fn config(e: Env, user: Address, config: ConfigData) {
        PriceOracle::config(&e, user, config)
    }

    pub fn add_assets(e: Env, user: Address, assets: Vec<Address>) {
        PriceOracle::add_assets(&e, user, assets)
    }

    pub fn set_price(e: Env, user: Address, updates: Vec<i128>, timestamp: u64) {
        PriceOracle::set_price(&e, user, updates, timestamp)
    }

    //end of admin section

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
        PriceOracle::price(&e, asset, timestamp)
    }

    pub fn lastprice(e: Env, asset: Address) -> Option<PriceData> {
        PriceOracle::lastprice(&e, asset)
    }

    pub fn x_price(
        e: Env,
        base_asset: Address,
        quote_asset: Address,
        timestamp: u64,
    ) -> Option<PriceData> {
        PriceOracle::x_price(&e, base_asset, quote_asset, timestamp)
    }

    pub fn x_lt_price(e: Env, base_asset: Address, quote_asset: Address) -> Option<PriceData> {
        PriceOracle::x_lt_price(&e, base_asset, quote_asset)
    }

    pub fn prices(e: Env, asset: Address, records: u32) -> Option<Vec<PriceData>> {
        PriceOracle::prices(&e, asset, records)
    }

    pub fn x_prices(
        e: Env,
        base_asset: Address,
        quote_asset: Address,
        records: u32,
    ) -> Option<Vec<PriceData>> {
        PriceOracle::x_prices(&e, base_asset, quote_asset, records)
    }

    pub fn twap(e: Env, asset: Address, records: u32) -> Option<i128> {
        PriceOracle::twap(&e, asset, records)
    }

    pub fn x_twap(e: Env, base_asset: Address, quote_asset: Address, records: u32) -> Option<i128> {
        PriceOracle::x_twap(&e, base_asset, quote_asset, records)
    }
}