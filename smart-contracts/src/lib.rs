#![no_std]

use soroban_sdk::{contractimpl, symbol, Address, Env};

pub struct GreenFlow;

#[contractimpl]
impl GreenFlow {
    pub fn hello(env: Env, _caller: Address) -> String {
        let _ = env;
        "GreenFlow contract initialized".to_string()
    }
}
