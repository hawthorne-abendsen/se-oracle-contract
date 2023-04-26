use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    AlreadyInitialized = 0,
    InvalidResolution = 1,
    Unauthorized = 2,
    InvalidTimestamp = 3,
    PriceNotFound = 4,
    InvalidAssetPair = 5,
    AssetAlreadyPresented = 6,
    InvalidUpdatesLength = 8,
    InvalidPriceValue = 9,
    NoPrevPrice = 10,
    InvalidFeeAsset = 11,
    DepositNotEnabled = 12,
    InvalidDepositAmount = 13,
    InvalidFreeResolution = 14,
    InsufficientBalance = 15,
}