use ic_cdk::api::caller;
use ic_cdk::export::candid::Nat;
use token_wallet::*;

#[test]
fn test_transfer() {
    // Simulate a transfer from caller to another account
    transfer(caller(), id(), 500).unwrap();
    assert_eq!(balance_of(caller()).unwrap(), Nat::from(500));
    assert_eq!(balance_of(id()).unwrap(), Nat::from(500));
}

#[test]
fn test_insufficient_balance() {
    // Attempt transfer with insufficient balance
    let result = transfer(caller(), id(), 1001);
    assert_eq!(result, Err(TokenError::InsufficientBalance));
}

#[test]
fn test_transfer_to_self() {
    // Attempt transfer to own account
    let result = transfer(caller(), caller(), 100);
    assert_eq!(result, Err(TokenError::TransferToSelf));
}

#[test]
fn test_balance_of() {
    // Test balance_of function
    assert_eq!(balance_of(caller()).unwrap(), Nat::from(1000));
}
