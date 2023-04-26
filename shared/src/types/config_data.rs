use soroban_sdk::{contracttype, Address, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigData {
    pub admin: Address,
    pub period: u64,
    pub assets: Vec<Address>,
    pub base_fee: i128
}
