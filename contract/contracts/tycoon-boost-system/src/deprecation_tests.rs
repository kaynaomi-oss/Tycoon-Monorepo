/// # Deprecated Function Coverage (#1041)
///
/// `get_boosts` and `prune_expired_boosts` are deprecated in favor of
/// `get_active_boosts` and automatic pruning via `add_boost`, but remain
/// callable for backward compatibility. Both emit `DeprecatedFunctionCalledEvent`
/// so integrations still depending on them can be tracked and migrated.
///
/// Functional correctness of these two functions (pruning behavior, return
/// values) is already covered in `advanced_integration_tests.rs` and
/// `time_boundary_tests.rs`. This file covers what no other file does: the
/// deprecation-tracking event itself.
#[cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{testutils::Events, Address, Env};

fn setup(env: &Env) -> (TycoonBoostSystemClient, Address) {
    let contract_id = env.register(TycoonBoostSystem, ());
    let client = TycoonBoostSystemClient::new(env, &contract_id);
    let admin = Address::generate(env);
    env.mock_all_auths();
    client.initialize(&admin);
    (client, admin)
}

fn nb(id: u128, value: u32) -> Boost {
    Boost {
        id,
        boost_type: BoostType::Additive,
        value,
        priority: 0,
        expires_at_ledger: 0,
    }
}

/// Calling the deprecated `get_boosts` emits `DeprecatedFunctionCalledEvent`.
#[test]
fn deprecated_get_boosts_emits_event() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let player = Address::generate(&env);
    client.add_boost(&player, &nb(1, 1000));

    let before = env.events().all().len();
    client.get_boosts(&player);
    let after = env.events().all().len();

    assert!(after > before, "get_boosts must emit a deprecation event");
}

/// Calling the deprecated `prune_expired_boosts` emits `DeprecatedFunctionCalledEvent`,
/// independent of whether anything was actually pruned.
#[test]
fn deprecated_prune_expired_boosts_emits_event() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let player = Address::generate(&env);
    client.add_boost(&player, &nb(1, 1000));

    let before = env.events().all().len();
    client.prune_expired_boosts(&player);
    let after = env.events().all().len();

    assert!(
        after > before,
        "prune_expired_boosts must emit a deprecation event"
    );
}

/// Each call to a deprecated function emits its own event — migration
/// tracking depends on every call being counted, not just the first.
#[test]
fn deprecated_function_emits_event_on_every_call() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let player = Address::generate(&env);
    client.add_boost(&player, &nb(1, 1000));

    let before = env.events().all().len();
    client.get_boosts(&player);
    let mid = env.events().all().len();
    client.get_boosts(&player);
    let after = env.events().all().len();

    assert!(mid > before, "first call must emit an event");
    assert!(after > mid, "second call must also emit an event");
}

/// `get_active_boosts` (the non-deprecated replacement) does not emit a
/// deprecation event — confirms the tracking event is scoped only to the
/// deprecated functions, not a false positive on every storage read.
#[test]
fn non_deprecated_replacement_emits_no_deprecation_event() {
    let env = Env::default();
    let (client, _) = setup(&env);
    let player = Address::generate(&env);
    client.add_boost(&player, &nb(1, 1000));

    let before = env.events().all().len();
    client.get_active_boosts(&player);
    let after = env.events().all().len();

    assert_eq!(
        after, before,
        "get_active_boosts must not emit a deprecation event"
    );
}
