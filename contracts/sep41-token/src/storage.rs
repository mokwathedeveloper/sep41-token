use soroban_sdk::{contracttype, Address};

#[contracttype]
pub struct AllowanceKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub live_until_ledger: u32,
}

#[contracttype]
pub enum DataKey {
    Balance(Address),
    Allowance(AllowanceKey),
    TotalSupply,
    Admin,
    Cap,
    Frozen(Address),
}
