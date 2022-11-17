#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Accounts, vec, BytesN, Env, Vec};

fn generate_assets(env: &Env, count: u8) -> Vec<BytesN<32>> {
    let mut assets = Vec::new(env);
    for i in 0..count {
        assets.push_back(BytesN::from_array(&env, &[i; 32]));
    }
    assets
}

#[test]
fn test() {
    let env = &Env::default();
    let assets = generate_assets(env, 2);
    let asset1 = assets.get(0).unwrap().ok().unwrap();
    let asset2 = assets.get(1).unwrap().ok().unwrap();

    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, OracleContract);
    let client = OracleContractClient::new(&env, &contract_id);

    //try to get price before initialization
    let result = client.get_price(&Identifier::Contract(asset1.clone()));
    assert_eq!(result, Option::None);

    let updates = vec![
        &env,
        AssetPriceUpdate {
            asset: Identifier::Contract(asset1.clone()),
            price: 100,
        },
        AssetPriceUpdate {
            asset: Identifier::Contract(asset2.clone()),
            price: 200,
        },
    ];

    let admin_account = env.accounts().generate();

    let admin_address = Address::Account(admin_account.clone());

    //set admin
    client
        .with_source_account(&admin_account)
        .set_admin(&admin_address);

    assert_eq!(client.get_admin().unwrap(), admin_address);

    //get admin nonce
    let nonce = client.with_source_account(&admin_account).nonce();

    //set prices for assets
    client
        .with_source_account(&admin_account)
        .set_price(&nonce, &updates);
    //check prices
    let mut result = client.get_price(&Identifier::Contract(asset1.clone()));
    assert_ne!(result, Option::None);
    assert_eq!(
        result,
        Option::Some(vec![
            &env,
            100,
            0
        ])
    );

    result = client.get_price(&Identifier::Contract(asset2.clone()));
    assert_ne!(result, Option::None);
    assert_eq!(
        result,
        Option::Some(vec![
            &env,
            200,
            0,
        ])
    );

    //try to get price for unknown asset
    result = client.get_price(&Identifier::Contract(BytesN::from_array(&env, &[3; 32])));
    assert_eq!(result, Option::None);
}

#[test]
#[should_panic]
fn unauthorized() {
    let env = &Env::default();
    let assets = generate_assets(env, 2);
    let asset1 = assets.get(0).unwrap().ok().unwrap();
    let asset2 = assets.get(1).unwrap().ok().unwrap();

    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, OracleContract);
    let client = OracleContractClient::new(&env, &contract_id);

    let updates = vec![
        &env,
        AssetPriceUpdate {
            asset: Identifier::Contract(asset1),
            price: 100,
        },
        AssetPriceUpdate {
            asset: Identifier::Contract(asset2),
            price: 200,
        },
    ];

    let account = env.accounts().generate();
    let nonce = BigInt::from_u32(&env, 0);
    //set prices for assets
    client
        .with_source_account(&account)
        .set_price(&nonce, &updates);
}
