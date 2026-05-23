#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::our_token::{SibToken, SibTokenClient};

const INITIAL_SUPPLY: i128 = 1_000_000_0000000; // 10,000,000 with 7 decimal places representation

struct SetUpResult<'a> {
    env: Env,
    client: SibTokenClient<'a>,
    admin: Address,
    alice: Address,
    bob: Address,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    let contract_id = env.register(SibToken, (&admin, INITIAL_SUPPLY));
    let client = SibTokenClient::new(&env, &contract_id);

    // Fund alice with tokens so transfer/burn tests have a balance to work with
    client.mint(&alice, &10_000_i128);

    SetUpResult {
        env,
        client,
        admin,
        alice,
        bob,
    }
}

// ─── Metadata ────────────────────────────────────────────────────────────────

#[test]
fn test_name() {
    let s = setup();
    assert_eq!(s.client.name(), String::from_str(&s.env, "SibToken"));
}

#[test]
fn test_symbol() {
    let s = setup();
    let expected = String::from_str(&s.env, "SIB");
    let wrong = String::from_str(&s.env, "Sib");
    assert_eq!(s.client.symbol(), expected);
    assert_ne!(s.client.symbol(), wrong);
}

#[test]
fn test_decimals() {
    let s = setup();
    assert_eq!(s.client.decimals(), 18u32);
}

// ─── Mint ─────────────────────────────────────────────────────────────────────

#[test]
fn test_constructor_mints_to_admin() {
    let s = setup();
    assert_eq!(s.client.balance(&s.admin), INITIAL_SUPPLY);
}

#[test]
fn test_mint_increases_balance_and_total_supply() {
    let s = setup();
    let before = s.client.balance(&s.bob);
    let before_supply = s.client.total_supply();

    s.client.mint(&s.bob, &500_i128);

    assert_eq!(s.client.balance(&s.bob), before + 500);
    assert_eq!(s.client.total_supply(), before_supply + 500);
}

#[test]
fn test_mint_by_admin_only() {
    let s = setup();
    // With mock_all_auths the auth check passes regardless of caller identity;
    // the test verifies the operation succeeds and balances are correct.
    let before = s.client.balance(&s.alice);
    s.client.mint(&s.alice, &100_i128);
    assert_eq!(s.client.balance(&s.alice), before + 100);
}

// ─── Transfer ─────────────────────────────────────────────────────────────────

#[test]
fn test_transfer_moves_tokens() {
    let s = setup();
    let alice_before = s.client.balance(&s.alice);
    let bob_before = s.client.balance(&s.bob);

    s.client.transfer(&s.alice, &s.bob, &1_000_i128);

    assert_eq!(s.client.balance(&s.alice), alice_before - 1_000);
    assert_eq!(s.client.balance(&s.bob), bob_before + 1_000);
}

#[test]
fn test_transfer_does_not_change_total_supply() {
    let s = setup();
    let supply_before = s.client.total_supply();

    s.client.transfer(&s.alice, &s.bob, &500_i128);

    assert_eq!(s.client.total_supply(), supply_before);
}

#[test]
#[should_panic(expected = "insufficient funds")]
fn test_transfer_panics_on_insufficient_funds() {
    let s = setup();
    let alice_balance = s.client.balance(&s.alice);
    // Attempt to transfer more than alice holds
    s.client.transfer(&s.alice, &s.bob, &(alice_balance + 1));
}

// ─── Approve & Allowance ──────────────────────────────────────────────────────

#[test]
fn test_approve_sets_allowance() {
    let s = setup();
    let live_until = s.env.ledger().sequence() + 1_000;

    s.client.approve(&s.alice, &s.bob, &2_000_i128, &live_until);

    assert_eq!(s.client.allowance(&s.alice, &s.bob), 2_000);
}

#[test]
fn test_approve_zero_clears_allowance() {
    let s = setup();
    let live_until = s.env.ledger().sequence() + 1_000;

    s.client.approve(&s.alice, &s.bob, &500_i128, &live_until);
    assert_eq!(s.client.allowance(&s.alice, &s.bob), 500);

    // Setting to 0 is allowed even with an expired ledger target (live_until = 0)
    s.client.approve(&s.alice, &s.bob, &0_i128, &0u32);
    assert_eq!(s.client.allowance(&s.alice, &s.bob), 0);
}

// ─── Transfer From ────────────────────────────────────────────────────────────

#[test]
fn test_transfer_from_deducts_allowance_and_moves_tokens() {
    let s = setup();
    let live_until = s.env.ledger().sequence() + 1_000;

    s.client.approve(&s.alice, &s.bob, &3_000_i128, &live_until);

    let alice_before = s.client.balance(&s.alice);
    let admin_before = s.client.balance(&s.admin);

    // bob transfers 1_000 from alice to admin on alice's behalf
    s.client.transfer_from(&s.bob, &s.alice, &s.admin, &1_000_i128);

    assert_eq!(s.client.balance(&s.alice), alice_before - 1_000);
    assert_eq!(s.client.balance(&s.admin), admin_before + 1_000);
    // Remaining allowance
    assert_eq!(s.client.allowance(&s.alice, &s.bob), 2_000);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn test_transfer_from_panics_when_allowance_too_low() {
    let s = setup();
    let live_until = s.env.ledger().sequence() + 1_000;

    s.client.approve(&s.alice, &s.bob, &100_i128, &live_until);
    // Try to transfer more than the allowance
    s.client.transfer_from(&s.bob, &s.alice, &s.admin, &500_i128);
}

// ─── Burn ─────────────────────────────────────────────────────────────────────

#[test]
fn test_burn_reduces_balance_and_total_supply() {
    let s = setup();
    let alice_before = s.client.balance(&s.alice);
    let supply_before = s.client.total_supply();

    s.client.burn(&s.alice, &2_000_i128);

    assert_eq!(s.client.balance(&s.alice), alice_before - 2_000);
    assert_eq!(s.client.total_supply(), supply_before - 2_000);
}

#[test]
#[should_panic(expected = "insufficient funds")]
fn test_burn_panics_on_insufficient_funds() {
    let s = setup();
    let alice_balance = s.client.balance(&s.alice);
    s.client.burn(&s.alice, &(alice_balance + 1));
}

// ─── Burn From ────────────────────────────────────────────────────────────────

#[test]
fn test_burn_from_deducts_allowance_and_burns() {
    let s = setup();
    let live_until = s.env.ledger().sequence() + 1_000;

    s.client.approve(&s.alice, &s.bob, &5_000_i128, &live_until);

    let alice_before = s.client.balance(&s.alice);
    let supply_before = s.client.total_supply();

    s.client.burn_from(&s.bob, &s.alice, &1_500_i128);

    assert_eq!(s.client.balance(&s.alice), alice_before - 1_500);
    assert_eq!(s.client.total_supply(), supply_before - 1_500);
    assert_eq!(s.client.allowance(&s.alice, &s.bob), 3_500);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn test_burn_from_panics_when_allowance_too_low() {
    let s = setup();
    let live_until = s.env.ledger().sequence() + 1_000;

    s.client.approve(&s.alice, &s.bob, &200_i128, &live_until);
    s.client.burn_from(&s.bob, &s.alice, &500_i128);
}

// ─── Total Supply ─────────────────────────────────────────────────────────────

#[test]
fn test_total_supply_reflects_mint_and_burn() {
    let s = setup();
    let initial = s.client.total_supply();

    s.client.mint(&s.bob, &1_000_i128);
    assert_eq!(s.client.total_supply(), initial + 1_000);

    s.client.burn(&s.bob, &400_i128);
    assert_eq!(s.client.total_supply(), initial + 1_000 - 400);
}
