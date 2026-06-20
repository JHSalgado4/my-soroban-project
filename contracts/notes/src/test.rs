#![cfg(test)]

use soroban_sdk::{Env, Address};
use crate::{LockPayContract, LockPayContractClient};

#[test]
fn test_happy_path() {
    let e = Env::default();
    let contract_id = e.register_contract(None, LockPayContract);
    let client = LockPayContractClient::new(&e, &contract_id);

    let user = Address::random(&e);

    e.ledger().set_timestamp(1000);

    client.deposit(&user, &50, &2000);

    e.ledger().set_timestamp(3000);

    let amount = client.withdraw(&user);

    assert_eq!(amount, 50);
}

#[test]
#[should_panic]
fn test_early_withdraw() {
    let e = Env::default();
    let contract_id = e.register_contract(None, LockPayContract);
    let client = LockPayContractClient::new(&e, &contract_id);

    let user = Address::random(&e);

    e.ledger().set_timestamp(1000);

    client.deposit(&user, &50, &2000);

    client.withdraw(&user); // should fail
}

#[test]
fn test_state_after_deposit() {
    let e = Env::default();
    let contract_id = e.register_contract(None, LockPayContract);
    let client = LockPayContractClient::new(&e, &contract_id);

    let user = Address::random(&e);

    e.ledger().set_timestamp(1000);

    client.deposit(&user, &75, &2000);

    let lock = client.get_lock(&user);

    assert_eq!(lock.amount, 75);
}

#[test]
#[should_panic]
fn test_unlock_time_past() {
    let e = Env::default();
    let contract_id = e.register_contract(None, LockPayContract);
    let client = LockPayContractClient::new(&e, &contract_id);

    let user = Address::random(&e);

    e.ledger().set_timestamp(1000);

    client.deposit(&user, &50, &900); // invalid
}

#[test]
fn test_withdraw_removes_lock() {
    let e = Env::default();
    let contract_id = e.register_contract(None, LockPayContract);
    let client = LockPayContractClient::new(&e, &contract_id);

    let user = Address::random(&e);

    e.ledger().set_timestamp(1000);
    client.deposit(&user, &50, &2000);

    e.ledger().set_timestamp(3000);
    client.withdraw(&user);

    let result = std::panic::catch_unwind(|| {
        client.get_lock(&user);
    });

    assert!(result.is_err());
}