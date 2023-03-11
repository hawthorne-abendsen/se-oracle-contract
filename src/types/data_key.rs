use soroban_sdk::contracttype;

use super::asset_price_key::AssetPriceKey;

#[contracttype]
pub enum DataKey {
    Admin,
    Base,
    Price(AssetPriceKey),
    Timestamp,
    Decimals,
    RdmPeriod,
    Resolution,
    Assets,
}
