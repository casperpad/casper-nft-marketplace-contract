//! Implementation of purse.

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{Key, URef, U512};

use crate::{
    constants::{PURSE_BALANCE_KEY_NAME, PURSE_KEY_NAME},
    detail,
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
