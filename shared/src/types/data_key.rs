use soroban_sdk::{contracttype, BytesN};

use super::asset_price_key::AssetPriceKey;

#[contracttype]
pub enum DataKey {
    Admin,
    Price(AssetPriceKey),
    Timestamp,
    RdmPeriod,
    Assets,
    BaseFee,
    Balance(BytesN<32>)
}
