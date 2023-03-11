use soroban_sdk::{panic_with_error, Address, Env, Vec};

use crate::types;

use types::{
    asset_price_key::AssetPriceKey, data_key::DataKey, error::Error, price_data::PriceData,
};

use super::i128_extensions;

use i128_extensions::I128Extensions;

pub trait EnvExtensions {
    fn is_authorized(&self, invoker: &Address) -> bool;

    fn is_initialized(&self) -> bool;

    fn get_admin(&self) -> Option<Address>;

    fn set_admin(&self, admin: &Address);

    fn get_base(&self) -> Option<Address>;

    fn set_base(&self, base: Address);

    fn get_price(&self, asset: Address, timestamp: u64) -> Option<i128>;

    fn set_price(&self, asset: Address, price: i128, timestamp: u64);

    fn get_last_timestamp(&self) -> Option<u64>;

    fn set_last_timestamp(&self, timestamp: u64);

    fn get_decimals(&self) -> Option<u32>;

    fn set_decimals(&self, decimals: u32);

    fn get_rdm_period(&self) -> Option<u64>;

    fn set_rdm_period(&self, period: u64);

    fn get_resolution(&self) -> Option<u32>;

    fn set_resolution(&self, resolution: u32);

    fn get_assets(&self) -> Vec<Address>;

    fn set_assets(&self, assets: Vec<Address>);

    fn get_prices(&self, asset: Address, rounds: u32) -> Option<Vec<PriceData>>;

    fn get_x_price(&self, base_asset: Address, quote_asset: Address, timestamp: u64)
        -> Option<i128>;

    fn get_x_prices(
        self,
        base_asset: Address,
        quote_asset: Address,
        rounds: u32,
    ) -> Option<Vec<PriceData>>;
}

impl EnvExtensions for Env {

    fn is_authorized(&self, invoker: &Address) -> bool {
        invoker.require_auth();

        //invoke get_admin to check if the admin is set
        let admin = self.get_admin();
        if admin.is_none() {
            return false;
        }
        invoker == &admin.unwrap()
    }

    fn is_initialized(&self) -> bool {
        self.storage().has(&DataKey::Admin)
    }

    fn get_admin(&self) -> Option<Address> {
        if !self.storage().has(&DataKey::Admin) {
            return None;
        }
        let admin = self.storage().get_unchecked(&DataKey::Admin).unwrap();
        Some(admin)
    }

    fn set_admin(&self, admin: &Address) {
        self.storage().set(&DataKey::Admin, admin);
    }

    fn get_base(&self) -> Option<Address> {
        if !self.storage().has(&DataKey::Base) {
            return None;
        }
        let base = self.storage().get_unchecked(&DataKey::Base).unwrap();
        Some(base)
    }

    fn set_base(&self, base: Address) {
        self.storage().set(&DataKey::Base, &base);
    }

    fn get_price(&self, asset: Address, timestamp: u64) -> Option<i128> {
        //build the key for the price
        let data_key = DataKey::Price(AssetPriceKey { asset, timestamp });

        //check if the price is available
        if !self.storage().has(&data_key) {
            return Option::None;
        }

        //get the price
        Option::Some(self.storage().get_unchecked(&data_key).unwrap())
    }

    fn set_price(&self, asset: Address, price: i128, timestamp: u64) {
        //build the key for the price
        let data_key = DataKey::Price(AssetPriceKey { asset, timestamp });

        //set the price
        self.storage().set(&data_key, &price);
    }

    fn get_last_timestamp(&self) -> Option<u64> {
        //check if the marker is available
        if !self.storage().has(&DataKey::Timestamp) {
            return Option::None;
        }

        //get the marker
        Option::Some(self.storage().get_unchecked(&DataKey::Timestamp).unwrap())
    }

    fn set_last_timestamp(&self, timestamp: u64) {
        self.storage().set(&DataKey::Timestamp, &timestamp);
    }

    fn get_assets(&self) -> Vec<Address> {
        if !self.storage().has(&DataKey::Assets) {
            //return empty vector
            return Vec::new(&self);
        }
        self.storage().get_unchecked(&DataKey::Assets).unwrap()
    }

    fn set_assets(&self, assets: Vec<Address>) {
        self.storage().set(&DataKey::Assets, &assets);
    }

    fn get_x_price(
        &self,
        base_asset: Address,
        quote_asset: Address,
        timestamp: u64,
    ) -> Option<i128> {
        //get decimals
        let decimals = self.get_decimals();
        if decimals.is_none() {
            return Option::None;
        }

        get_x_price(&self, &base_asset, &quote_asset, timestamp, decimals.unwrap())
    }

    fn get_prices(&self, asset: Address, rounds: u32) -> Option<Vec<PriceData>> {
        let timeframe = self.get_resolution();
        if timeframe.is_none() {
            return Option::None;
        }

        let decimals = self.get_decimals();
        if decimals.is_none() {
            return Option::None;
        }
        prices(
            &self,
            |timestamp| self.get_price(asset.clone(), timestamp),
            rounds,
            timeframe.unwrap().into(),
        )
    }

    fn get_x_prices(
        self,
        base_asset: Address,
        quote_asset: Address,
        rounds: u32,
    ) -> Option<Vec<PriceData>> {
        let timeframe = self.get_resolution();
        if timeframe.is_none() {
            return Option::None;
        }

        let decimals = self.get_decimals();
        if decimals.is_none() {
            return Option::None;
        }

        prices(
            &self,
            |timestamp| get_x_price(&self, &base_asset, &quote_asset, timestamp, decimals.unwrap()),
            rounds,
            timeframe.unwrap().into(),
        )
    }

    fn get_decimals(&self) -> Option<u32> {
        if !self.storage().has(&DataKey::Decimals) {
            return Option::None;
        }
        Option::Some(self.storage().get_unchecked(&DataKey::Decimals).unwrap())
    }

    fn set_decimals(&self, decimals: u32) {
        self.storage().set(&DataKey::Decimals, &decimals);
    }

    fn get_rdm_period(&self) -> Option<u64> {
        if !self.storage().has(&DataKey::RdmPeriod) {
            return Option::None;
        }
        Option::Some(self.storage().get_unchecked(&DataKey::RdmPeriod).unwrap())
    }

    fn set_rdm_period(&self, rdm_period: u64) {
        self.storage().set(&DataKey::RdmPeriod, &rdm_period);
    }

    fn get_resolution(&self) -> Option<u32> {
        if !self.storage().has(&DataKey::Resolution) {
            return Option::None;
        }
        Option::Some(self.storage().get_unchecked(&DataKey::Resolution).unwrap())
    }

    fn set_resolution(&self, resolution: u32) {
        self.storage().set(&DataKey::Resolution, &resolution);
    }
}

fn prices<F: Fn(u64) -> Option<i128>>(
    e: &Env,
    get_price_fn: F,
    rounds: u32,
    timeframe: u64,
) -> Option<Vec<PriceData>> {
    //check if the asset is valid
    let mut timestamp = e.get_last_timestamp().unwrap_or(0);
    if timestamp == 0 {
        return None;
    }

    let mut prices = Vec::new(&e);

    for _ in 0..rounds {
        let price = get_price_fn(timestamp);
        if price.is_none() {
            //TODO: should we put None here?
            continue;
        }
        prices.push_back(PriceData {
            price: price.unwrap(),
            timestamp,
        });
        timestamp -= timeframe;
    }

    if prices.len() == 0 {
        return Option::None;
    }

    Some(prices)
}

fn get_x_price(
    e: &Env,
    base_asset: &Address,
    quote_asset: &Address,
    timestamp: u64,
    decimals: u32,
) -> Option<i128> {
    //check if the asset are the same
    if base_asset == quote_asset {
        panic_with_error!(e, Error::InvalidAssetPair);
    }

    //get the price for base_asset
    let base_asset_price = e.get_price(base_asset.clone(), timestamp);
    if base_asset_price.is_none() {
        return Option::None;
    }

    //get the price for quote_asset
    let quote_asset_price = e.get_price(quote_asset.clone(), timestamp);
    if quote_asset_price.is_none() {
        return Option::None;
    }

    //calculate the cross price
    Option::Some(
        base_asset_price
            .unwrap()
            .fixed_div_floor(quote_asset_price.unwrap(), decimals),
    )
}
