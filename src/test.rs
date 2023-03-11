#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

use crate::extensions::i128_extensions::I128Extensions;

struct InitData {
    admin: Address,
    base: Address,
    decimals: u32,
    resolution: u32,
    period: u64,
    assets: Vec<Address>
}

fn init_contract_with_admin() -> (Env, OracleContractClient, InitData) {

    let env = Env::default();

    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, OracleContract);
    let client = OracleContractClient::new(&env, &contract_id);

    let init_data = InitData {
        admin: Address::random(&env),
        base: Address::random(&env),
        decimals: 14,
        resolution: 300_000,
        period: 14,
        assets: generate_assets(&env, 10)
    };

    let admin = init_data.admin.clone();

    //set admin
    client
        .set_admin(&admin, &admin);

    //set base
    client
        .set_base(&admin, &init_data.base);

    //set decimals
    client
        .set_dcmals(&init_data.admin, &init_data.decimals);

    //set resolution
    client
        .set_rsltn(&init_data.admin, &init_data.resolution);

    //set period
    client
        .set_prd(&init_data.admin, &init_data.period);

    //add assets
    client
        .add_assets(&init_data.admin, &init_data.assets);

    (env, client, init_data)
}

fn normalize_price(price: i128, decimals: u32) -> i128 {
    price * 10i128.pow(decimals)
}

fn generate_assets(e: &Env, count: usize) -> Vec<Address> {
    let mut assets = Vec::new(&e);
    for _ in 0..count {
        assets.push_back(Address::random(&e));
    }
    assets
}

fn get_updates(env: &Env, assets: Vec<Address>, price: i128) -> Vec<i128> {
    let mut updates = Vec::new(&env);
    for _ in assets.iter() {
        updates.push_back(price);
    }
    updates
}

#[test]
fn init_test() {
    let (_, client, init_data) = init_contract_with_admin();

    let address = client.admin().unwrap();
    assert_eq!(address, init_data.admin.clone());

    let base = client.base().unwrap();
    assert_eq!(base, init_data.base.clone());

    let resolution = client.resolution().unwrap();
    assert_eq!(resolution, init_data.resolution / 1000);

    let period = client.period().unwrap();
    assert_eq!(period, init_data.period);

    let decimals = client.decimals().unwrap();
    assert_eq!(decimals, init_data.decimals);

    let assets = client.assets().unwrap();
    assert_eq!(assets, init_data.assets);
}

#[test]
fn last_price_test() {
    let (env, client, init_data) = init_contract_with_admin();

    let admin = &init_data.admin;
    let assets = init_data.assets;


    let timestamp = 600_000;
    let updates = get_updates(&env, assets.clone(), normalize_price(100, init_data.decimals));

    //set prices for assets
    client.set_price(&admin, &updates, &timestamp);

    let timestamp = 900_000;
    let updates = get_updates(&env, assets.clone(), normalize_price(200, init_data.decimals));

    //set prices for assets
    client.set_price(&admin, &updates, &timestamp);

    //check last prices
    let result = client.lastprice(&assets.get_unchecked(1).unwrap());
    assert_ne!(result, Option::None);
    assert_eq!(
        result,
        Option::Some(PriceData {
            price: normalize_price(200, init_data.decimals),
            timestamp: 900_000 as u64
        })
    );
}

#[test]
fn get_price_test() {
    let (env, client, init_data) = init_contract_with_admin();

    let admin = &init_data.admin;
    let assets = init_data.assets;

    let timestamp = 600_000;
    let updates = get_updates(&env, assets.clone(), normalize_price(100, init_data.decimals));

    client.set_price(&admin, &updates, &timestamp);

    let timestamp = 900_000;
    let updates = get_updates(&env, assets.clone(), normalize_price(200, init_data.decimals));

    client.set_price(&admin, &updates, &timestamp);

    //check last prices
    let mut result = client.lastprice(&assets.get_unchecked(1).unwrap());
    assert_ne!(result, Option::None);
    assert_eq!(
        result,
        Option::Some(PriceData {
            price: normalize_price(200, init_data.decimals),
            timestamp: 900_000 as u64
        })
    );

    //check price at 899_000
    result = client.price(&assets.get_unchecked(1).unwrap(), &899_000);
    assert_ne!(result, Option::None);
    assert_eq!(
        result,
        Option::Some(PriceData {
            price: normalize_price(100, init_data.decimals),
            timestamp: 600_000 as u64
        })
    );
}


#[test]
fn get_x_lt_price_test() {
    let (env, client, init_data) = init_contract_with_admin();

    let admin = &init_data.admin;
    let assets = init_data.assets;

    let timestamp = 600_000;
    let updates = get_updates(&env, assets.clone(), normalize_price(100, init_data.decimals));

    client.set_price(&admin, &updates, &timestamp);

    //check last x price
    let result = client.x_lt_price(&assets.get_unchecked(1).unwrap(), &assets.get_unchecked(2).unwrap());
    assert_ne!(result, Option::None);
    assert_eq!(
        result,
        Option::Some(PriceData {
            price: normalize_price(1, init_data.decimals),
            timestamp: 600_000 as u64
        })
    );
}


#[test]
fn get_x_price_test() {
    let (env, client, init_data) = init_contract_with_admin();

    let admin = &init_data.admin;
    let assets = init_data.assets;

    let timestamp = 600_000;
    let updates = get_updates(&env, assets.clone(), normalize_price(100, init_data.decimals));

    //set prices for assets
    client.set_price(&admin, &updates, &timestamp);

    let timestamp = 900_000;
    let updates = get_updates(&env, assets.clone(), normalize_price(200, init_data.decimals));

    //set prices for assets
    client.set_price(&admin, &updates, &timestamp);

    //check last prices
    let mut result = client.x_lt_price(&assets.get_unchecked(1).unwrap(), &assets.get_unchecked(2).unwrap());
    assert_ne!(result, Option::None);
    assert_eq!(
        result,
        Option::Some(PriceData {
            price: normalize_price(1, init_data.decimals),
            timestamp: 900_000 as u64
        })
    );

    //check price at 899_000
    result = client.x_price(&assets.get_unchecked(1).unwrap(), &assets.get_unchecked(2).unwrap(), &899_000);
    assert_ne!(result, Option::None);
    assert_eq!(
        result,
        Option::Some(PriceData {
            price: normalize_price(1, init_data.decimals),
            timestamp: 600_000 as u64
        })
    );
}

#[test]
fn twap_test() {
    let (env, client, init_data) = init_contract_with_admin();

    let admin = &init_data.admin;
    let assets = init_data.assets;

    let timestamp = 600_000;
    let updates = get_updates(&env, assets.clone(), normalize_price(100, init_data.decimals));

    //set prices for assets
    client.set_price(&admin, &updates, &timestamp);

    let timestamp = 900_000;
    let updates = get_updates(&env, assets.clone(), normalize_price(200, init_data.decimals));

    //set prices for assets
    client.set_price(&admin, &updates, &timestamp);

    let result = client.twap(&assets.get_unchecked(1).unwrap(), &2);

    assert_ne!(result, Option::None);
    assert_eq!(
        result.unwrap(),
        normalize_price(150, init_data.decimals)
    );
}


#[test]
fn x_twap_test() {
    let (env, client, init_data) = init_contract_with_admin();

    let admin = &init_data.admin;
    let assets = init_data.assets;

    let timestamp = 600_000;
    let updates = get_updates(&env, assets.clone(), normalize_price(100, init_data.decimals));

    //set prices for assets
    client.set_price(&admin, &updates, &timestamp);

    let timestamp = 900_000;
    let updates = get_updates(&env, assets.clone(), normalize_price(200, init_data.decimals));

    //set prices for assets
    client.set_price(&admin, &updates, &timestamp);

    let result = client.x_twap(&assets.get_unchecked(1).unwrap(), &assets.get_unchecked(2).unwrap(), &2);

    assert_ne!(result, Option::None);
    assert_eq!(
        result.unwrap(),
        normalize_price(1, init_data.decimals)
    );
}

#[test]
fn get_non_registered_asset_price_test() {
    let (env, client, _) = init_contract_with_admin();


    //try to get price for unknown asset
    let result = client.lastprice(&Address::random(&env));
    assert_eq!(result, Option::None);
}

#[test]
#[should_panic]
fn unauthorized_test() {
    let (env, client, init_data) = init_contract_with_admin();

    let assets = init_data.assets;

    let updates = get_updates(&env, assets, 100);

    let account = Address::random(&env);
    let timestamp = (112331 as u64).get_normalized_timestamp(init_data.resolution.into());
    //set prices for assets
    client.set_price(&account, &updates, &timestamp);
}

#[test]
fn div_test() {
    let a = i128::MAX;
    let b = i128::MAX / 42;
    let result = a.fixed_div_floor(b, 14);
    assert_eq!(result, 4200000000000000);
}

// #[test]
// fn update_test() {
//     let (client, env, admin) = init_contract_with_admin();
//     let assets = generate_assets(&env, 50);
//     let mut timestamp = 0;
//     for i in 0..100 {
//         timestamp += TIMEFRAME;
//         let updates = get_updates(&env, assets.clone(), normalize_price(i));
//         client.set_price(&admin, &updates, &timestamp);
//         env.budget().reset();
//     }
//     //get first address from assets array
//     let asset = assets.first().unwrap().ok().unwrap();

//     let prices = client.prices(&asset, &10);
//     assert_ne!(prices, Option::None);
//     assert_eq!(prices.unwrap().len(), 10);
// }