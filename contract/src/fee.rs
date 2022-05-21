//! Implementation of treasury wallet.

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{URef, U512};

use crate::{constants::FEE_KEY_NAME, detail};

#[inline]
pub(crate) fn fee_uref() -> URef {
    detail::get_uref(FEE_KEY_NAME)
}

/// Reads a treasury wallet.
pub(crate) fn read_fee() -> U512 {
    let uref = fee_uref();
    storage::read(uref).unwrap_or_revert().unwrap_or_default()
}

/// Writes a treasury wallet.
pub(crate) fn write_fee(value: U512) {
    let uref = fee_uref();
    storage::write(uref, value);
}
