//! Implementation of owner.

use alloc::vec::Vec;
use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{account::AccountHash, ContractHash, URef, U256};

use crate::{constants::ON_OFFERS_KEY_NAME, detail};

pub type OnOffer = (ContractHash, U256, AccountHash);

#[inline]
pub(crate) fn on_offers_uref() -> URef {
    detail::get_uref(ON_OFFERS_KEY_NAME)
}

pub(crate) fn read_on_offers() -> Vec<OnOffer> {
    let uref = on_offers_uref();
    storage::read(uref).unwrap_or_revert().unwrap_or_default()
}

pub(crate) fn write_on_offers(value: Vec<OnOffer>) {
    let uref = on_offers_uref();
    storage::write(uref, value);
}

pub(crate) fn find(
    collection: ContractHash,
    token_id: U256,
    offerer: AccountHash,
) -> Option<usize> {
    let on_offers = read_on_offers();
    let result = on_offers.iter().position(|on_offer| {
        on_offer.0.eq(&collection) && on_offer.1.eq(&token_id) && on_offer.2.eq(&offerer)
    });
    result
}
