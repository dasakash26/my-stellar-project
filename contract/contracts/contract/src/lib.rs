#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Address};
#[contract]
pub struct PiggyBank;

#[contracttype]
enum DataKey {
    Balance,
    Owner,
}

#[contractimpl]
impl PiggyBank {
    // initialize contract with owner
    pub fn init(env: Env, owner: Address) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Balance, &0i128);
    }

    // deposit amount
    pub fn deposit(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let mut bal: i128 = env.storage().instance().get(&DataKey::Balance).unwrap();
        bal += amount;

        env.storage().instance().set(&DataKey::Balance, &bal);
    }

    // view balance
    pub fn get_balance(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Balance).unwrap()
    }

    // withdraw (only owner)
    pub fn withdraw(env: Env, to: Address, amount: i128) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();

        let mut bal: i128 = env.storage().instance().get(&DataKey::Balance).unwrap();

        if amount > bal {
            panic!("insufficient funds");
        }

        bal -= amount;
        env.storage().instance().set(&DataKey::Balance, &bal);
    }
}
