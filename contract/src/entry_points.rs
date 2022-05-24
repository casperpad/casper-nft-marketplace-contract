use alloc::{string::String, vec, vec::Vec};
use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Parameter,
};

use crate::constants::{
    AMOUNT_RUNTIME_ARG_NAME, BUY_ORDER_ENTRY_NAME, CANCEL_OFFER_ENTRY_NAME,
    CANCEL_ORDER_ENTRY_NAME, COLLECTION_RUNTIME_ARG_NAME, CONSTRUCTOR_ENTRY_NAME,
    CREATE_OFFER_ENTRY_NAME, CREATE_ORDER_ENTRY_NAME, GET_PURSE_ENTRY_NAME,
    ORDER_ID_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, PRICE_RUNTIME_ARG_NAME,
    SET_TREASURY_WALLET_ENTRY_NAME, TOKEN_ID_RUNTIME_ARG_NAME, TRANSFER_OWNERSHIP_ENTRY_NAME,
};

/// Returns the `constructor` entry point.
pub fn constructor() -> EntryPoint {
    EntryPoint::new(
        String::from(CONSTRUCTOR_ENTRY_NAME),
        vec![],
        CLType::Unit,
        EntryPointAccess::Groups(vec![Group::new(CONSTRUCTOR_ENTRY_NAME)]),
        EntryPointType::Contract,
    )
}

/// Returns the `set_treasury_wallet` entry point.
pub fn set_treasury_wallet() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_TREASURY_WALLET_ENTRY_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `create_order` entry point.
pub fn create_order() -> EntryPoint {
    EntryPoint::new(
        String::from(CREATE_ORDER_ENTRY_NAME),
        vec![
            Parameter::new(COLLECTION_RUNTIME_ARG_NAME, CLType::Key),
            Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, CLType::U256),
            Parameter::new(PRICE_RUNTIME_ARG_NAME, CLType::U256),
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `cancel_order` entry point.
pub fn cancel_order() -> EntryPoint {
    EntryPoint::new(
        String::from(CANCEL_ORDER_ENTRY_NAME),
        vec![Parameter::new(ORDER_ID_RUNTIME_ARG_NAME, CLType::U256)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `buy_order` entry point.
pub fn buy_order() -> EntryPoint {
    EntryPoint::new(
        String::from(BUY_ORDER_ENTRY_NAME),
        vec![Parameter::new(ORDER_ID_RUNTIME_ARG_NAME, CLType::U256)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `create_offer` entry point.
pub fn create_offer() -> EntryPoint {
    EntryPoint::new(
        String::from(CREATE_OFFER_ENTRY_NAME),
        vec![
            Parameter::new(COLLECTION_RUNTIME_ARG_NAME, CLType::Key),
            Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, CLType::U256),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, CLType::U512),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `cancel_offer` entry point.
pub fn cancel_offer() -> EntryPoint {
    EntryPoint::new(
        String::from(CANCEL_OFFER_ENTRY_NAME),
        vec![
            Parameter::new(COLLECTION_RUNTIME_ARG_NAME, CLType::Key),
            Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `get_purse` entry point.
pub fn get_purse() -> EntryPoint {
    EntryPoint::new(
        String::from(GET_PURSE_ENTRY_NAME),
        vec![],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `transfer_ownership` entry point.
pub fn transfer_ownership() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_OWNERSHIP_ENTRY_NAME),
        vec![Parameter::new(OWNER_RUNTIME_ARG_NAME, CLType::Key)],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(constructor());
    entry_points.add_entry_point(transfer_ownership());
    entry_points.add_entry_point(set_treasury_wallet());
    entry_points.add_entry_point(get_purse());
    entry_points.add_entry_point(create_order());
    entry_points.add_entry_point(cancel_order());
    entry_points.add_entry_point(create_offer());
    entry_points.add_entry_point(cancel_offer());
    entry_points.add_entry_point(buy_order());
    entry_points
}
