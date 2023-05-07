use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    Unauthorized = 1,
    AssetAlreadyPresented = 3,
    InvalidUpdatesLength = 4,
    InvalidPriceValue = 5,
    NoPrevPrice = 6,
    InvalidFeeAsset = 11,
    InvalidDepositAmount = 12,
    InsufficientBalance = 13,
}