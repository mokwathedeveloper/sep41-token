# SibToken — SEP-41 Token on Stellar / Soroban

A fully compliant **SEP-41** fungible token smart contract written in Rust for the
[Soroban](https://developers.stellar.org/docs/build/smart-contracts) smart-contract
platform on the Stellar network.

---

## Table of Contents

- [Overview](#overview)
- [Contract Functions](#contract-functions)
  - [Constructor](#constructor)
  - [Mint](#mint)
  - [Transfer](#transfer)
  - [Transfer From](#transfer-from)
  - [Burn](#burn)
  - [Burn From](#burn-from)
  - [Approve / Allowance](#approve--allowance)
  - [Metadata](#metadata)
- [SEP-41 Trait Compliance](#sep-41-trait-compliance)
- [Events](#events)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Build](#build)
- [Test](#test)
- [Deploy to Testnet](#deploy-to-testnet)
- [Deployed Contract](#deployed-contract)

---

## Overview

**SibToken (SIB)** is a fungible token that implements the
[SEP-41 token interface](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md).
The contract supports minting, burning, and delegated transfers via an allowance
mechanism — fully consistent with the Stellar token standard.

| Property | Value |
|----------|-------|
| Token Name | `SibToken` |
| Symbol | `SIB` |
| Decimals | `18` |
| Initial Supply | Set at deployment via constructor |

---

## Contract Functions

### Constructor

Called **once** automatically when the contract is deployed. Mints the entire
`initial_supply` to the `admin` address.

```rust
pub fn __constructor(env: Env, admin: Address, initial_supply: i128)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `admin` | `Address` | Account that owns admin privileges (can call `mint`) |
| `initial_supply` | `i128` | Total tokens to create and assign to `admin` on deploy |

---

### Mint

Creates new tokens and assigns them to `to`. **Only the admin** address set at
deployment may call this function.

```rust
pub fn mint(env: Env, to: Address, amount: i128)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `to` | `Address` | Recipient of the newly minted tokens |
| `amount` | `i128` | Number of tokens to mint |

- Increases `to`'s balance by `amount`.
- Increases `total_supply` by `amount`.
- Emits a `Mint` event.

---

### Transfer

Transfers `amount` tokens from `from` to `to`. Requires authorization from `from`.

```rust
fn transfer(env: Env, from: Address, to: Address, amount: i128)
```

- Panics with `"insufficient funds"` if `from`'s balance is below `amount`.
- Emits a `Transfer` event.

---

### Transfer From

Transfers `amount` tokens from `from` to `to` using `spender`'s pre-approved
allowance. Requires authorization from `spender`.

```rust
fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128)
```

- Panics with `"insufficient allowance"` if `spender`'s allowance from `from` is below `amount`.
- Panics with `"insufficient funds"` if `from`'s balance is below `amount`.
- Deducts `amount` from the stored allowance before transferring.
- Emits a `Transfer` event.

---

### Burn

Destroys `amount` tokens from `from`. Requires authorization from `from`.

```rust
fn burn(env: Env, from: Address, amount: i128)
```

- Panics with `"insufficient funds"` if `from`'s balance is below `amount`.
- Decreases `total_supply` by `amount`.
- Emits a `Burn` event.

---

### Burn From

Destroys `amount` tokens from `from` using `spender`'s pre-approved allowance.
Requires authorization from `spender`.

```rust
fn burn_from(env: Env, spender: Address, from: Address, amount: i128)
```

- Panics with `"insufficient allowance"` if `spender`'s allowance from `from` is below `amount`.
- Panics with `"insufficient funds"` if `from`'s balance is below `amount`.
- Deducts `amount` from the stored allowance and decreases `total_supply`.
- Emits a `Burn` event.

---

### Approve / Allowance

**`approve`** — Grants `spender` the right to transfer or burn up to `amount`
tokens from `from`'s account until `live_until_ledger`.

```rust
fn approve(env: Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32)
```

**`allowance`** — Returns the current approved amount for a (from, spender) pair.
Returns `0` if the allowance has expired.

```rust
fn allowance(env: Env, from: Address, spender: Address) -> i128
```

---

### Metadata

| Function | Returns | Description |
|----------|---------|-------------|
| `name(env)` | `String` | Token name — `"SibToken"` |
| `symbol(env)` | `String` | Token symbol — `"SIB"` |
| `decimals(env)` | `u32` | Decimal places — `18` |
| `total_supply(env)` | `i128` | Current total token supply |
| `balance(env, id)` | `i128` | Token balance of `id` |

---

## SEP-41 Trait Compliance

The contract struct `SibToken` implements the `TokenInterface` trait:

```rust
#[contractimpl]
impl TokenInterface for SibToken { ... }
```

This ensures every function in the SEP-41 standard is present with the correct
signature, making the contract compatible with any Stellar tooling or DApp that
expects a SEP-41 token.

---

## Events

| Event | Topics | Data |
|-------|--------|------|
| `Mint` | `[admin: Address, to: Address]` | `amount: i128` |
| `Transfer` | `[from: Address, to: Address]` | `amount: i128` |
| `Burn` | `[from: Address]` | `amount: i128` |
| `Approval` | `[from: Address, spender: Address]` | `amount: i128, live_until_ledger: u32` |

---

## Project Structure

```
sep41-token/
├── Cargo.toml                        # Workspace manifest
├── contracts/
│   └── sep41-token/
│       ├── Cargo.toml
│       ├── Makefile
│       └── src/
│           ├── lib.rs                # Module declarations
│           ├── our_token.rs          # Contract implementation (SibToken)
│           ├── token_trait.rs        # SEP-41 TokenInterface trait definition
│           ├── storage.rs            # Storage keys and value types
│           ├── events.rs             # On-chain event structs
│           ├── error.rs              # Contract error codes
│           └── test.rs               # Unit & integration tests
└── README.md
```

---

## Prerequisites

| Tool | Install |
|------|---------|
| Rust + `wasm32v1-none` target | `rustup target add wasm32v1-none` |
| Stellar CLI | [Installation guide](https://developers.stellar.org/docs/tools/developer-tools/stellar-cli) |
| Funded testnet account | `stellar keys generate alice --network testnet --fund` |

---

## Build

```bash
cd contracts/sep41-token
stellar contract build
```

The compiled WASM is written to:
```
target/wasm32v1-none/release/sep41_token.wasm
```

Or use the Makefile shortcut from the workspace root:

```bash
make build
```

---

## Test

Run all unit and integration tests:

```bash
make test
# or directly:
cargo test
```

### Test coverage

| Test | Function covered |
|------|-----------------|
| `test_name` | `name()` |
| `test_symbol` | `symbol()` |
| `test_decimals` | `decimals()` |
| `test_constructor_mints_to_admin` | `__constructor` / `mint` (on deploy) |
| `test_mint_increases_balance_and_total_supply` | `mint()` |
| `test_mint_by_admin_only` | `mint()` auth |
| `test_transfer_moves_tokens` | `transfer()` |
| `test_transfer_does_not_change_total_supply` | `transfer()` supply invariant |
| `test_transfer_panics_on_insufficient_funds` | `transfer()` error path |
| `test_approve_sets_allowance` | `approve()` / `allowance()` |
| `test_approve_zero_clears_allowance` | `approve()` reset |
| `test_transfer_from_deducts_allowance_and_moves_tokens` | `transfer_from()` |
| `test_transfer_from_panics_when_allowance_too_low` | `transfer_from()` error path |
| `test_burn_reduces_balance_and_total_supply` | `burn()` |
| `test_burn_panics_on_insufficient_funds` | `burn()` error path |
| `test_burn_from_deducts_allowance_and_burns` | `burn_from()` |
| `test_burn_from_panics_when_allowance_too_low` | `burn_from()` error path |
| `test_total_supply_reflects_mint_and_burn` | `total_supply()` invariant |

---

## Deploy to Testnet

### 1. Build the contract

```bash
make build
```

### 2. Generate / fund a testnet account (skip if you already have one)

```bash
stellar keys generate alice --network testnet --fund
```

### 3. Get the admin's public address

```bash
stellar keys address alice
```

Copy the output — you will pass it as `--admin` in the next step.

### 4. Deploy with constructor arguments

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/sep41_token.wasm \
  --network testnet \
  --source alice \
  -- \
  --admin <ADMIN_PUBLIC_KEY> \
  --initial-supply 10000000000000
```

> The `--` separator is required to distinguish CLI flags from constructor arguments.  
> `10000000000000` = 10,000,000 tokens (18 decimals).

The command prints the **contract ID** on success. Save it for subsequent invocations.

### 5. Verify deployment

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  --source alice \
  -- name
```

Expected output: `"SibToken"`

### 6. Invoke contract functions (examples)

**Check balance:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> --network testnet --source alice \
  -- balance --id <ADDRESS>
```

**Mint tokens:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> --network testnet --source alice \
  -- mint --to <RECIPIENT_ADDRESS> --amount 1000000
```

**Transfer tokens:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> --network testnet --source alice \
  -- transfer --from <FROM_ADDRESS> --to <TO_ADDRESS> --amount 500000
```

---

## Deployed Contract

| Network | Contract ID |
|---------|-------------|
| Testnet | *(paste contract ID here after deployment)* |

---

## Assignment Reference

This contract satisfies the **Stellar Impact Bootcamp — Week 2, Day 2** requirements:

- [x] Implement `transfer_from`
- [x] Implement `burn`
- [x] Implement `burn_from`
- [x] Implement `mint` (with constructor mint on deployment)
- [x] Apply `TokenInterface` trait for SEP-41 compliance
- [x] Complete test suite covering all functions
- [x] Deploy to Stellar testnet
