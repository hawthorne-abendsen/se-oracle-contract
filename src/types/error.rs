use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    IncorrectNonce = 1,
    Unauthorized = 2,
    InvalidAddressType = 3,
    InvalidTimestamp = 4,
    PriceNotFound = 5,
    InvalidAssetPair = 6,
    AssetAlreadyAdded = 7,
    NoAssetsFound = 8,
    InvalidUpdatesLength = 9,
    InvalidUpdate = 10,
}
