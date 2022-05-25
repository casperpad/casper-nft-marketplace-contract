//! Implementation of allowances.
use alloc::{string::String, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, ContractHash, Key, URef, U256};

use crate::{constants::ORDERS_KEY_NAME, detail, Order};

#[inline]
pub(crate) fn orders_uref() -> URef {
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
pub(crate) fn write_order(order: Order) {
    let dictionary_item_key = make_dictionary_item_key(order.collection, order.token_id);
    let orders_uref = orders_uref();
    storage::dictionary_put(orders_uref, &dictionary_item_key, order);
}

/// Reads an allowance for a owner and spender
pub(crate) fn read_order(collection: ContractHash, token_id: U256) -> Order {
    let dictionary_item_key = make_dictionary_item_key(collection, token_id);
    let orders_uref = orders_uref();
    storage::dictionary_get(orders_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_revert()
}
