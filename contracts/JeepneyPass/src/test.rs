#![cfg(test)]

use soroban_sdk::{
    testutils::Address as _,
    Address, Env,
};

use crate::{JeepneyPassContract, JeepneyPassContractClient};

mod tests {

    use super::*;

    #[test]
    fn test_happy_path() {
        let env = Env::default();

        let id = env.register(JeepneyPassContract, ());
        let client = JeepneyPassContractClient::new(&env, &id);

        let passenger = Address::generate(&env);
        let driver = Address::generate(&env);
        let operator = Address::generate(&env);

        client.pay_fare(&passenger, &driver, &operator, &15);

        assert_eq!(client.driver_total(), 12);
        assert_eq!(client.operator_total(), 3);
    }

    #[test]
    fn test_multiple_payments() {
        let env = Env::default();

        let id = env.register(JeepneyPassContract, ());
        let client = JeepneyPassContractClient::new(&env, &id);

        let passenger = Address::generate(&env);
        let driver = Address::generate(&env);
        let operator = Address::generate(&env);

        client.pay_fare(&passenger, &driver, &operator, &15);
        client.pay_fare(&passenger, &driver, &operator, &15);

        assert_eq!(client.driver_total(), 24);
        assert_eq!(client.operator_total(), 6);
    }

    #[test]
    fn test_state_verification() {
        let env = Env::default();

        let id = env.register(JeepneyPassContract, ());
        let client = JeepneyPassContractClient::new(&env, &id);

        let passenger = Address::generate(&env);
        let driver = Address::generate(&env);
        let operator = Address::generate(&env);

        client.pay_fare(&passenger, &driver, &operator, &20);

        assert_eq!(client.driver_total(), 16);
        assert_eq!(client.operator_total(), 4);
    }

    #[test]
    fn test_zero_before_payment() {
        let env = Env::default();

        let id = env.register(JeepneyPassContract, ());
        let client = JeepneyPassContractClient::new(&env, &id);

        assert_eq!(client.driver_total(), 0);
        assert_eq!(client.operator_total(), 0);
    }

    #[test]
    fn test_small_payment() {
        let env = Env::default();

        let id = env.register(JeepneyPassContract, ());
        let client = JeepneyPassContractClient::new(&env, &id);

        let passenger = Address::generate(&env);
        let driver = Address::generate(&env);
        let operator = Address::generate(&env);

        client.pay_fare(&passenger, &driver, &operator, &5);

        assert!(client.driver_total() > 0);
    }
}