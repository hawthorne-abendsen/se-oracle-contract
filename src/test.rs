#![cfg(test)]

use super::*;
use ed25519_dalek::Keypair;
use rand::thread_rng;

use soroban_auth::{Ed25519Signature, SignaturePayload, SignaturePayloadV0};
use soroban_sdk::{testutils::ed25519::Sign, vec, BytesN, Env, IntoVal, RawVal, Symbol, Vec};

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn make_identifier(e: &Env, kp: &Keypair) -> Identifier {
    Identifier::Ed25519(kp.public.to_bytes().into_val(e))
}

fn make_signature(e: &Env, contract_id: &BytesN<32>, kp: &Keypair, function: &str, args: Vec<RawVal>) -> Signature {
    let msg = SignaturePayload::V0(SignaturePayloadV0 {
        function: Symbol::from_str(function),
        contract: contract_id.clone(),
        network: e.ledger().network_passphrase(),
        args,
    });
    Signature::Ed25519(Ed25519Signature {
        public_key: BytesN::from_array(e, &kp.public.to_bytes()),
        signature: kp.sign(msg).unwrap().into_val(e),
    })
}

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
    assert_eq!(result, AssetPrice::None);

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

    let admin_kp = generate_keypair();
    let admin_id = make_identifier(&env, &admin_kp);
    //set admin
    client.init(&admin_id);
    //get admin nonce
    let nonce = client.nonce(&admin_id);

    let sig = make_signature(
        &env,
        &contract_id,
        &admin_kp,
        "set_price",
        vec![
            &env,
            admin_id.into_val(&env),
            nonce.clone().into_val(&env),
            updates.to_raw(),
        ],
    );

    //set prices for assets
    client.set_price(&sig, &nonce, &updates);

    //check prices
    let mut result = client.get_price(&Identifier::Contract(asset1.clone()));
    assert_ne!(result, AssetPrice::None);
    assert_eq!(
        result,
        AssetPrice::AssetPrice(AssetPriceData {
            price: 100,
            timestamp: 0,
        })
    );

    result = client.get_price(&Identifier::Contract(asset2.clone()));
    assert_ne!(result, AssetPrice::None);
    assert_eq!(
        result,
        AssetPrice::AssetPrice(AssetPriceData {
            price: 200,
            timestamp: 0,
        })
    );

    //try to get price for unknown asset
    result = client.get_price(&Identifier::Contract(BytesN::from_array(&env, &[3; 32])));
    assert_eq!(result, AssetPrice::None);
}

#[test]
#[should_panic(expected = "Not authorized by admin")]
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

    let user_kp = generate_keypair();
    let user_id = make_identifier(&env, &user_kp);

    let nonce = client.nonce(&user_id);

    let sig = make_signature(
        &env,
        &contract_id,
        &user_kp,
        "set_price",
        vec![
            &env,
            user_id.into_val(&env),
            nonce.clone().into_val(&env),
            updates.to_raw(),
        ],
    );

    //set prices for assets
    client.set_price(&sig, &nonce, &updates);
}