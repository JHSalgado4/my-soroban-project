#![no_std]

use soroban_sdk::{contractimpl, contracttype, symbol_short, Env, Address, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Lock {
    pub owner: Address,
    pub amount: i128,
    pub unlock_time: u64,
}

#[contracttype]
pub enum DataKey {
    Lock(Address),
}

pub struct LockPayContract;

#[contractimpl]
impl LockPayContract {

    // Deposit funds and lock until a specific time
    pub fn deposit(e: Env, user: Address, amount: i128, unlock_time: u64) {
        user.require_auth();

        let current_time = e.ledger().timestamp();

        // Unlock must be in future
        if unlock_time <= current_time {
            panic!("Unlock time must be in future");
        }

        let lock = Lock {
            owner: user.clone(),
            amount,
            unlock_time,
        };

        e.storage().instance().set(&DataKey::Lock(user), &lock);
    }

    // Withdraw funds after unlock time
    pub fn withdraw(e: Env, user: Address) -> i128 {
        user.require_auth();

        let current_time = e.ledger().timestamp();

        let key = DataKey::Lock(user.clone());

        let lock: Lock = e.storage().instance().get(&key).unwrap();

        if current_time < lock.unlock_time {
            panic!("Funds are still locked");
        }

        // Remove lock after withdrawal
        e.storage().instance().remove(&key);

        lock.amount
    }

    // View locked funds
    pub fn get_lock(e: Env, user: Address) -> Lock {
        let key = DataKey::Lock(user);
        e.storage().instance().get(&key).unwrap()
    }
}