use soroban_sdk::{Address, Env, String};

pub trait TokenInterface {
    /// Returns the allowance for `spender` to transfer from `from`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens to be drawn from.
    /// - `spender` - The address spending the tokens held by `from`.
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    /// Set the allowance by `amount` for `spender` to transfer/burn from `from`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens to be drawn from.
    /// - `spender` - The address being authorized to spend the tokens held by `from`.
    /// - `amount` - The tokens to be made available to `spender`.
    /// - `live_until_ledger` - The ledger number where this allowance expires. Cannot be
    ///   less than the current ledger number unless the amount is being set to 0.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["approve", from: Address, spender: Address]`
    /// - data - `[amount: i128, live_until_ledger: u32]`
    fn approve(env: Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32);

    /// Returns the balance of `id`.
    ///
    /// # Arguments
    ///
    /// - `id` - The address for which a balance is being queried.
    fn balance(env: Env, id: Address) -> i128;

    /// Transfer `amount` from `from` to `to`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens which will be withdrawn from.
    /// - `to` - The address which will receive the transferred tokens.
    /// - `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["transfer", from: Address, to: Address]`
    /// - data - `amount: i128`
    fn transfer(env: Env, from: Address, to: Address, amount: i128);

    /// Transfer `amount` from `from` to `to`, consuming the allowance of `spender`.
    ///
    /// # Arguments
    ///
    /// - `spender` - The address authorizing the transfer and having its allowance consumed.
    /// - `from` - The address holding the balance of tokens which will be withdrawn from.
    /// - `to` - The address which will receive the transferred tokens.
    /// - `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["transfer", from: Address, to: Address]`
    /// - data - `amount: i128`
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);

    /// Burn `amount` from `from`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens which will be burned.
    /// - `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["burn", from: Address]`
    /// - data - `amount: i128`
    fn burn(env: Env, from: Address, amount: i128);

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    ///
    /// # Arguments
    ///
    /// - `spender` - The address authorizing the burn and having its allowance consumed.
    /// - `from` - The address holding the balance of tokens which will be burned.
    /// - `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["burn", from: Address]`
    /// - data - `amount: i128`
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);

    /// Returns the number of decimals used to represent amounts of this token.
    fn decimals(env: Env) -> u32;

    /// Returns the name for this token.
    fn name(env: Env) -> String;

    /// Returns the symbol for this token.
    fn symbol(env: Env) -> String;
}
