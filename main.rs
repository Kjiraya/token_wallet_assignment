// Import necessary libraries
mod lib;
use ic_cdk::api::{caller, id};
use ic_cdk::export::candid::{CandidType, Nat};
use ic_cdk_macros::*;
use std::collections::HashMap;
use std::sync::Mutex;

// Define custom error type for token operations
#[derive(CandidType, Debug)]
enum TokenError {
    InsufficientBalance,
    TransferToSelf,
    NonExistentRecipient,
}

// Define token balance storage
static mut BALANCES: Option<Mutex<HashMap<PrincipalId, u64>>> = None;

// Initialize the token wallet
#[init]
fn init() {
    let mut balances = HashMap::new();
    balances.insert(caller(), 1000); // Initial balance for the caller
    unsafe {
        BALANCES = Some(Mutex::new(balances));
    }
}

// Query function to get balance of an account
#[query]
fn balance_of(owner: PrincipalId) -> Result<Nat, TokenError> {
    unsafe {
        if let Some(balances) = &BALANCES {
            let guard = balances.lock().unwrap();
            if let Some(balance) = guard.get(&owner) {
                Ok(Nat::from(*balance))
            } else {
                Err(TokenError::NonExistentRecipient)
            }
        } else {
            Err(TokenError::NonExistentRecipient)
        }
    }
}

// Update function to transfer tokens to another account
#[update]
fn transfer(to: PrincipalId, amount: u64) -> Result<(), TokenError> {
    let from = caller();

    // Ensure sender and recipient are different
    if from == to {
        return Err(TokenError::TransferToSelf);
    }

    unsafe {
        if let Some(balances) = &BALANCES {
            let mut guard = balances.lock().unwrap();
            if let Some(sender_balance) = guard.get_mut(&from) {
                if *sender_balance >= amount {
                    *sender_balance -= amount;
                    if let Some(recipient_balance) = guard.get_mut(&to) {
                        *recipient_balance += amount;
                        Ok(())
                    } else {
                        guard.insert(to, amount);
                        Ok(())
                    }
                } else {
                    Err(TokenError::InsufficientBalance)
                }
            } else {
                Err(TokenError::NonExistentRecipient)
            }
        } else {
            Err(TokenError::NonExistentRecipient)
        }
    }
}

// Entry point for the canister
#[export_name = "canister_update token_wallet"]

fn main() {
    
}
