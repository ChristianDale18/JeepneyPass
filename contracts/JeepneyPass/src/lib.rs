#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env,
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    TotalDriver,
    TotalOperator,
}

#[contract]
pub struct JeepneyPassContract;

#[contractimpl]
impl JeepneyPassContract {

    // Record a fare payment and split revenue
    pub fn pay_fare(
        env: Env,
        passenger: Address,
        driver: Address,
        operator: Address,
        fare: i128,
    ) {
        passenger.require_auth();

        let operator_cut = fare / 5; // 20%
        let driver_cut = fare - operator_cut;

        let driver_total: i128 = env.storage()
            .persistent()
            .get(&DataKey::TotalDriver)
            .unwrap_or(0);

        let operator_total: i128 = env.storage()
            .persistent()
            .get(&DataKey::TotalOperator)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::TotalDriver, &(driver_total + driver_cut));

        env.storage()
            .persistent()
            .set(&DataKey::TotalOperator, &(operator_total + operator_cut));

        // Store latest recipients
        env.storage().persistent().set(&driver, &driver_cut);
        env.storage().persistent().set(&operator, &operator_cut);
    }

    // View accumulated driver earnings
    pub fn driver_total(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::TotalDriver)
            .unwrap_or(0)
    }

    // View accumulated operator earnings
    pub fn operator_total(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::TotalOperator)
            .unwrap_or(0)
    }
}