//! Implementation of treasury wallet.

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{account::AccountHash, URef};

use crate::{constants::TREASURY_WALLET_KEY_NAME, detail};

#[inline]
pub(crate) fn treasury_wallet_uref() -> URef {
    detail::get_uref(TREASURY_WALLET_KEY_NAME)
}

/// Reads a treasury wallet.
pub(crate) fn read_treasury_wallet() -> AccountHash {
    let uref = treasury_wallet_uref();
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes a treasury wallet.
pub(crate) fn write_treasury_wallet(value: AccountHash) {
    let uref = treasury_wallet_uref();
    storage::write(uref, value);
}
