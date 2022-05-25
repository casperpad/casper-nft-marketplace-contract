//! Implementation of owner.

use alloc::vec::Vec;
use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ContractHash, URef, U256};

use crate::{constants::ON_ORDERS_KEY_NAME, detail};

pub type OnOrder = (ContractHash, U256);

#[inline]
pub(crate) fn on_orders_uref() -> URef {
    detail::get_uref(ON_ORDERS_KEY_NAME)
}

pub(crate) fn read_on_orders() -> Vec<OnOrder> {
    let uref = on_orders_uref();
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

pub(crate) fn write_on_orders(value: Vec<OnOrder>) {
    let uref = on_orders_uref();
    storage::write(uref, value);
}

pub(crate) fn find(collection: ContractHash, token_id: U256) -> Option<usize> {
    let on_orders = read_on_orders();
    let result = on_orders
        .iter()
        .position(|on_order| on_order.0.eq(&collection) && on_order.1.eq(&token_id));
    result
}
