//! Implementation of allowances.
use alloc::{string::String, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, ContractHash, Key, URef, U256};

use crate::{constants::ORDERS_KEY_NAME, detail, Offer};

#[inline]
pub(crate) fn offers_uref() -> URef {
    detail::get_uref(ORDERS_KEY_NAME)
}

/// Creates a dictionary item key for an (owner, spender) pair.
fn make_dictionary_item_key(collection: ContractHash, token_id: U256) -> String {
    let mut preimage = Vec::new();
    preimage.append(&mut Key::from(collection).to_bytes().unwrap_or_revert());
    preimage.append(&mut token_id.to_bytes().unwrap_or_revert());

    let key_bytes = runtime::blake2b(&preimage);
    hex::encode(&key_bytes)
}

/// Writes an allowance for owner and spender for a specific amount.
pub(crate) fn write_offer(offer: Offer) {
    let dictionary_item_key = make_dictionary_item_key(offer.collection, offer.token_id);
    let offers_uref = offers_uref();
    storage::dictionary_put(offers_uref, &dictionary_item_key, offer);
}

/// Reads an allowance for a owner and spender
pub(crate) fn read_offer(collection: ContractHash, token_id: U256) -> Option<Offer> {
    let dictionary_item_key = make_dictionary_item_key(collection, token_id);
    let offers_uref = offers_uref();
    storage::dictionary_get(offers_uref, &dictionary_item_key).unwrap_or_revert()
}
