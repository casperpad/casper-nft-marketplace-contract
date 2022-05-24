//! Implementation of purse.

use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{account::AccountHash, Key, URef, U512};

use crate::{
    constants::{AMOUNT_RUNTIME_ARG_NAME, PURSE_BALANCE_KEY_NAME, PURSE_KEY_NAME},
    detail,
    error::Error,
    fee, treasury_wallet,
};

/// Sets main purse which handle CSPR.
pub(crate) fn set_main_purse(purse: URef) {
    runtime::put_key(PURSE_KEY_NAME, Key::from(purse))
}
/// Reads main purse
pub(crate) fn get_main_purse() -> URef {
    detail::get_uref(PURSE_KEY_NAME)
}

pub(crate) fn read_purse_balance() -> U512 {
    let purse_balance_uref = detail::get_uref(PURSE_BALANCE_KEY_NAME);
    storage::read(purse_balance_uref)
        .unwrap_or_revert()
        .unwrap_or_default()
}

pub(crate) fn write_purse_balance(balance: U512) {
    let purse_balance_uref = detail::get_uref(PURSE_BALANCE_KEY_NAME);
    storage::write(purse_balance_uref, balance);
}

/// Check the entry point called from call and return new purse balance, otherwise revert
pub(crate) fn checked_balance() -> U512 {
    let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let purse: URef = get_main_purse();
    let new_purse_balance = system::get_purse_balance(purse).unwrap_or_default();
    let old_purse_balance = read_purse_balance();

    if !old_purse_balance
        .checked_add(amount)
        .unwrap_or_default()
        .eq(&new_purse_balance)
    {
        // entrypoint is called directly
        runtime::revert(Error::PermissionDenied);
    }
    update_purse_balance();
    new_purse_balance
}

pub(crate) fn transfer(account: AccountHash, amount: U512) {
    let purse: URef = get_main_purse();
    system::transfer_from_purse_to_account(purse, account, amount, None).unwrap_or_revert();
    update_purse_balance();
}
/// Send CSPR to account and treasury wallet
pub(crate) fn transfer_with_fee(account: AccountHash, amount: U512) {
    let fee = fee::read_fee();
    let transfer_amount_to_account = amount
        .checked_mul(U512::exp10(3).checked_sub(fee).unwrap_or_revert())
        .unwrap_or_revert()
        .checked_div(U512::exp10(3))
        .unwrap_or_revert();

    let transfer_amount_to_treasury_wallet = amount
        .checked_mul(fee)
        .unwrap_or_revert()
        .checked_div(U512::exp10(3))
        .unwrap_or_revert();
    let treasury_wallet = treasury_wallet::read_treasury_wallet();

    transfer(account, transfer_amount_to_account);
    transfer(treasury_wallet, transfer_amount_to_treasury_wallet);
}

pub(crate) fn update_purse_balance() {
    let purse: URef = get_main_purse();
    let new_purse_balance = system::get_purse_balance(purse).unwrap_or_default();
    write_purse_balance(new_purse_balance);
}
