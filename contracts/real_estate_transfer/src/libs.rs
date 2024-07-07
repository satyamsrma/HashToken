#![allow(non_snake_case)]
#![no_std]
#[warn(const_item_mutation)]

use soroban_sdk::{contract, contractimpl,Env};

// Define a global state for tracking used tokens
const USED_TOKENS: [u64; 100] = [0; 100]; // Example size, adjust as needed

#[contract]
pub struct TokenTransferManager;

#[contractimpl]
impl TokenTransferManager{
    pub fn transfer_if_not_used(_env:Env, token: u64)-> bool {
        // Check if the token has been used before
        if !is_token_used(token) {
            // Mark the token as used
            mark_token_as_used(token);
            
            true
            
            // Transfer 1 Lumen to the predefined wallet address
            // env.transfer_lumens(1, &env.constant("CBL3G5W3CIDH34GABQP5LK6D3IHZYTPE6X5EC2SLKN3BTAHB6ZFUEQB2")?)?;
            // let used_tokens_vec: Vec<u64> = USED_TOKENS.to_vec();
            // let used_tokens_str = format!("{:?}", used_tokens_vec);
            // log!(&env, "USED_TOKENS: {}", used_tokens_str);
        } 
        else {
            false
        }
    }
}

#[contractimpl]
impl TokenTransferManager {
    pub fn transfer_if_used(_env:Env, token: u64) {
        // Check if the token has been used before
        if is_token_used(token) {
            // Transfer 1 Lumen to the given wallet address
            // env.transfer_lumens(1, &env.caller())?;
        } 
    }
}

fn is_token_used(token: u64) -> bool {
    if USED_TOKENS[token as usize] == 1 {
        true
    } else {
        false
    }
}

fn mark_token_as_used(token: u64) {
    USED_TOKENS[token as usize] = 1;
}
