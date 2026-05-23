use soroban_sdk::{contract, contractimpl, Address, Env, String};

use crate::{
    events::{Approval, Burn, Mint, Transfer},
    storage::{AllowanceKey, AllowanceValue, DataKey},
    token_trait::TokenInterface,
};

#[contract]
pub struct SibToken;

/// Non-standard contract functions: constructor, mint, total_supply.
#[contractimpl]
impl SibToken {
    /// Called once on deployment. Mints `initial_supply` tokens to `admin`.
    pub fn __constructor(env: Env, admin: Address, initial_supply: i128) {
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage()
            .persistent()
            .set(&DataKey::Balance(admin.clone()), &initial_supply);
        env.storage()
            .persistent()
            .set(&DataKey::TotalSupply, &initial_supply);

        Mint {
            admin: admin.clone(),
            to: admin,
            amount: initial_supply,
        }
        .publish(&env);
    }

    /// Mint new tokens to `to`. Only the admin set at deployment may call this.
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .unwrap();
        admin.require_auth();

        let to_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);
        let total_supply: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(to_balance + amount));
        env.storage()
            .persistent()
            .set(&DataKey::TotalSupply, &(total_supply + amount));

        Mint {
            admin,
            to,
            amount,
        }
        .publish(&env);
    }

    /// Returns the total token supply.
    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }
}

/// SEP-41 Token Interface — ensures compliance with the Stellar token standard.
#[contractimpl]
impl TokenInterface for SibToken {
    fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let key = DataKey::Allowance(AllowanceKey {
            from,
            spender,
        });
        match env
            .storage()
            .persistent()
            .get::<DataKey, AllowanceValue>(&key)
        {
            Some(a) => {
                if a.live_until_ledger < env.ledger().sequence() {
                    0
                } else {
                    a.amount
                }
            }
            None => 0,
        }
    }

    fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) {
        from.require_auth();

        if amount > 0 && live_until_ledger < env.ledger().sequence() {
            panic!("live_until_ledger must be >= current ledger when amount > 0");
        }

        let key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender: spender.clone(),
        });

        env.storage().persistent().set(
            &key,
            &AllowanceValue {
                amount,
                live_until_ledger,
            },
        );

        Approval {
            from,
            spender,
            amount,
            live_until_ledger,
        }
        .publish(&env);
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let sender_balance = Self::balance(env.clone(), from.clone());
        let receiver_balance = Self::balance(env.clone(), to.clone());

        if sender_balance < amount {
            panic!("insufficient funds");
        }

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(sender_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(receiver_balance + amount));

        Transfer { from, to, amount }.publish(&env);
    }

    /// Transfer `amount` from `from` to `to` using `spender`'s allowance.
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        let current_allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if current_allowance < amount {
            panic!("insufficient allowance");
        }

        if amount > 0 {
            let key = DataKey::Allowance(AllowanceKey {
                from: from.clone(),
                spender: spender.clone(),
            });
            let stored: AllowanceValue = env.storage().persistent().get(&key).unwrap();
            env.storage().persistent().set(
                &key,
                &AllowanceValue {
                    amount: stored.amount - amount,
                    live_until_ledger: stored.live_until_ledger,
                },
            );
        }

        let sender_balance = Self::balance(env.clone(), from.clone());
        let receiver_balance = Self::balance(env.clone(), to.clone());

        if sender_balance < amount {
            panic!("insufficient funds");
        }

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(sender_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(receiver_balance + amount));

        Transfer { from, to, amount }.publish(&env);
    }

    /// Burn `amount` from `from`, reducing total supply.
    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let balance = Self::balance(env.clone(), from.clone());
        if balance < amount {
            panic!("insufficient funds");
        }

        let total_supply: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::TotalSupply, &(total_supply - amount));

        Burn { from, amount }.publish(&env);
    }

    /// Burn `amount` from `from` using `spender`'s allowance, reducing total supply.
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        let current_allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if current_allowance < amount {
            panic!("insufficient allowance");
        }

        if amount > 0 {
            let key = DataKey::Allowance(AllowanceKey {
                from: from.clone(),
                spender: spender.clone(),
            });
            let stored: AllowanceValue = env.storage().persistent().get(&key).unwrap();
            env.storage().persistent().set(
                &key,
                &AllowanceValue {
                    amount: stored.amount - amount,
                    live_until_ledger: stored.live_until_ledger,
                },
            );
        }

        let balance = Self::balance(env.clone(), from.clone());
        if balance < amount {
            panic!("insufficient funds");
        }

        let total_supply: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::TotalSupply, &(total_supply - amount));

        Burn { from, amount }.publish(&env);
    }

    fn decimals(_env: Env) -> u32 {
        18
    }

    fn name(env: Env) -> String {
        String::from_str(&env, "SibToken")
    }

    fn symbol(env: Env) -> String {
        String::from_str(&env, "SIB")
    }
}
