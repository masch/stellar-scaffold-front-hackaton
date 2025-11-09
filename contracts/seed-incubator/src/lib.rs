#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct TemperatureControl;

pub const ADMIN_KEY: &Symbol = &symbol_short!("ADMIN");

#[contractimpl]

impl TemperatureControl {
    pub fn __constructor(env: &Env, admin: Address) {
        // Require auth from the admin to make the transfer
        admin.require_auth();

        Self::set_admin(env, admin);
    }

    // Updates the temperature.
    // The temperature is an integer, representing the value in Celsius.
    // Only the admin account can call this function.
    pub fn update_temperature(env: &Env, new_temp: i32) {
        Self::require_admin(env);

        let temp_key = symbol_short!("TEMP");
        env.storage().instance().set(&temp_key, &new_temp);

        let admin = Self::admin(env).expect("admin not set");
        let topics = (Symbol::new(env, "temperature_updated"), admin.clone());

        env.events().publish(topics, new_temp);

        // env.events()
        //     .publish((temp_key, symbol_short!("updated")), new_temp);
    }

    // Gets the current temperature.
    pub fn get_temperature(env: Env) -> i32 {
        let temp_key = symbol_short!("TEMP");
        env.storage().instance().get(&temp_key).unwrap_or(0) // Returns 0 if not set
    }

    /// Get current admin
    pub fn admin(env: &Env) -> Option<Address> {
        env.storage().instance().get(ADMIN_KEY)
    }

    /// Private helper function to require auth from the admin
    fn require_admin(env: &Env) {
        let admin = Self::admin(env).expect("admin not set");
        admin.require_auth();
    }

    /// Set a new admin. Only callable by admin.
    pub fn set_admin(env: &Env, admin: Address) {
        // Check if admin is already set
        if env.storage().instance().has(ADMIN_KEY) {
            panic!("admin already set");
        }
        env.storage().instance().set(ADMIN_KEY, &admin);
    }
}
