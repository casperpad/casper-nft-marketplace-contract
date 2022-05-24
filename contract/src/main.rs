#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{
    collections::BTreeSet,
    string::{String, ToString},
    vec,
};
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, contracts::NamedKeys, runtime_args, CLValue, ContractHash,
    ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use constants::{
    AMOUNT_RUNTIME_ARG_NAME, COLLECTION_RUNTIME_ARG_NAME, CONSTRUCTOR_ENTRY_NAME,
    CONTRACT_NAME_KEY_NAME, FEE_KEY_NAME, FEE_RUNTIME_ARG_NAME, ORDERS_KEY_NAME,
    ORDER_ID_RUNTIME_ARG_NAME, OWNER_KEY_NAME, OWNER_RUNTIME_ARG_NAME, PRICE_RUNTIME_ARG_NAME,
    PURSE_BALANCE_KEY_NAME, TOKEN_ID_RUNTIME_ARG_NAME, TREASURY_WALLET_KEY_NAME,
};
use detail::store_result;
use error::Error;
use icep47::ICEP47;
use offer::{Bid, Offer};
use order::Order;

mod address;
mod constants;
mod detail;
mod entry_points;
mod error;
mod fee;
mod icep47;
mod offer;
mod offers;
mod order;
mod orders;
mod owner;
mod purse;
mod treasury_wallet;

/// Transfer ownership to `owner` parameter should be `account-hash-000...000`
#[no_mangle]
pub extern "C" fn transfer_ownership() {
    owner::only_owner();
    let new_owner_hash: AccountHash = {
        let owner: Key = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
        owner.into_account().unwrap()
    };
    let owner_uref: URef = owner::owner_uref();
    owner::write_owner_to(owner_uref, new_owner_hash);
}

#[no_mangle]
pub extern "C" fn set_treasury_wallet() {
    owner::only_owner();
    let new_treasury_wallet: AccountHash = {
        let new_owner_string: String = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
        AccountHash::from_formatted_str(new_owner_string.as_str()).unwrap()
    };
    treasury_wallet::write_treasury_wallet(new_treasury_wallet);
}

#[no_mangle]
pub extern "C" fn set_fee() {
    owner::only_owner();
    let fee: U512 = runtime::get_named_arg(FEE_RUNTIME_ARG_NAME);
    fee::write_fee(fee);
}

#[no_mangle]
pub extern "C" fn create_order() {
    let collection: ContractHash = {
        let collection_key: Key = runtime::get_named_arg(COLLECTION_RUNTIME_ARG_NAME);
        ContractHash::new(collection_key.into_hash().unwrap())
    };
    let token_id: U256 = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let price: U512 = runtime::get_named_arg(PRICE_RUNTIME_ARG_NAME);

    let maker = runtime::get_caller();
    let is_active = true;

    let me = detail::get_caller_address()
        .unwrap()
        .as_contract_package_hash()
        .unwrap()
        .clone();

    let caller = detail::get_immediate_caller_address()
        .unwrap()
        .as_account_hash()
        .unwrap()
        .clone();

    let approved = ICEP47::new(collection).get_approved(Key::from(caller), token_id);

    if approved != Some(Key::from(me)) {
        runtime::revert(Error::NotApproved);
    }

    ICEP47::new(collection).transfer_from(
        Key::from(runtime::get_caller()),
        Key::from(me),
        vec![token_id],
    );

    let token_owner = ICEP47::new(collection).owner_of(token_id);

    if token_owner != Some(Key::from(me)) {
        runtime::revert(Error::NotOwner);
    }

    let orders_length = orders::read_orders_length();
    let id = orders_length;

    let order = Order {
        id,
        collection,
        token_id,
        maker,
        price,
        is_active,
    };
    orders::write_order(order);

    let new_orders_length = orders_length.checked_add(U256::one()).unwrap();
    orders::write_orders_length(new_orders_length);
    runtime::ret(CLValue::from_t(id).unwrap());
}

#[no_mangle]
pub extern "C" fn change_order_price() {}

#[no_mangle]
pub extern "C" fn cancel_order() {
    let order_id: U256 = runtime::get_named_arg(ORDER_ID_RUNTIME_ARG_NAME);
    let mut order = orders::read_order(order_id);
    let caller = runtime::get_caller();
    if caller != order.maker {
        runtime::revert(Error::NotOrderMaker);
    }

    // Refund the token
    ICEP47::new(order.collection).transfer(Key::from(order.maker), vec![order.token_id]);
    order.is_active = false;
    orders::write_order(order);
    store_result(order);
}

#[no_mangle]
pub extern "C" fn buy_order() {
    let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let order_id: U256 = runtime::get_named_arg(ORDER_ID_RUNTIME_ARG_NAME);

    let _ = purse::checked_balance();
    let mut order = orders::read_order(order_id);
    if !amount.eq(&order.price) {
        runtime::revert(Error::NotValidAmount);
    }

    let caller = runtime::get_caller();
    if caller != order.maker {
        runtime::revert(Error::NotOrderMaker);
    }

    // Send NFT to caller
    ICEP47::new(order.collection).transfer(Key::from(caller), vec![order.token_id]);
    // Send CSPR to order maker and treasury wallet
    let fee = fee::read_fee();
    let transfer_amount_to_order_maker = order
        .price
        .checked_mul(U512::exp10(3).checked_sub(fee).unwrap_or_revert())
        .unwrap_or_revert()
        .checked_div(U512::exp10(3))
        .unwrap_or_revert();

    let transfer_amount_to_treasury_wallet = order
        .price
        .checked_mul(fee)
        .unwrap_or_revert()
        .checked_div(U512::exp10(3))
        .unwrap_or_revert();
    let treasury_wallet = treasury_wallet::read_treasury_wallet();

    purse::transfer(order.maker, transfer_amount_to_order_maker);
    purse::transfer(treasury_wallet, transfer_amount_to_treasury_wallet);

    order.is_active = false;
    orders::write_order(order);
}

#[no_mangle]
pub extern "C" fn create_offer() {
    let _ = purse::checked_balance();
    let collection: ContractHash = {
        let collection_key: Key = runtime::get_named_arg(COLLECTION_RUNTIME_ARG_NAME);
        ContractHash::new(collection_key.into_hash().unwrap())
    };
    let token_id: U256 = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let maker = runtime::get_caller();
    let price: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let offer_time = u64::from(runtime::get_blocktime());

    let bid = Bid {
        maker,
        price,
        offer_time,
    };

    match offers::read_offer(collection, token_id) {
        Some(mut offer) => {
            if offer.is_exist_bid(&bid) {
                runtime::revert(Error::BidExist);
            }

            if let Some(index) = offer.get_bid_index_by_account(maker) {
                offer.bids.remove(index);
            }

            // add bid to exist offer
            offer.bids.push(bid);
            store_result(offer.clone());
            offers::write_offer(offer);
        }
        None => {
            let is_active = true;

            let first_offer = Offer {
                collection,
                token_id,
                bids: vec![bid],
                is_active,
            };

            store_result(first_offer.clone());
            offers::write_offer(first_offer);
        }
    };
}

#[no_mangle]
pub extern "C" fn cancel_offer() {
    let collection: ContractHash = {
        let collection_key: Key = runtime::get_named_arg(COLLECTION_RUNTIME_ARG_NAME);
        ContractHash::new(collection_key.into_hash().unwrap())
    };
    let token_id: U256 = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let maker = runtime::get_caller();

    match offers::read_offer(collection, token_id) {
        Some(mut offer) => match offer.get_bid_index_by_account(maker) {
            Some(index) => {
                let bid = offer.bids.get(index).unwrap();
                //Refund
                purse::transfer(bid.maker, bid.price);
                offer.bids.remove(index);
                store_result(offer.clone());
                offers::write_offer(offer);
            }
            None => runtime::revert(Error::PermissionDenied),
        },
        None => runtime::revert(Error::PermissionDenied),
    };
}

#[no_mangle]
pub extern "C" fn accept_offer() {
    let offer_id: u8 = runtime::get_named_arg("name");
}

#[no_mangle]
pub extern "C" fn constructor() {
    let purse: URef = system::create_purse();
    purse::set_main_purse(purse);

    let purse_balance_uref = storage::new_uref(U512::zero());
    runtime::put_key(PURSE_BALANCE_KEY_NAME, purse_balance_uref.into());
}

#[no_mangle]
pub extern "C" fn get_purse() {
    let purse: URef = purse::get_main_purse();
    runtime::ret(CLValue::from_t(purse).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    // Set Contract owner
    let owner_key: Key = {
        let owner: AccountHash = runtime::get_caller();
        let owner_uref: URef = storage::new_uref(owner).into_read_write();
        Key::from(owner_uref)
    };

    let treasury_wallet_key: Key = {
        let treasury_wallet: AccountHash = runtime::get_caller();
        let treasury_wallet_uref: URef = storage::new_uref(treasury_wallet).into_read_write();
        Key::from(treasury_wallet_uref)
    };

    let fee_key: Key = {
        // Fee decimal is 3 here fee is 2.5%
        let fee = U512::from(25);
        let fee_uref: URef = storage::new_uref(fee).into_read_write();
        Key::from(fee_uref)
    };

    let orders_key: Key = {
        let uref = storage::new_dictionary(ORDERS_KEY_NAME).unwrap();
        Key::from(uref)
    };

    let mut named_keys = NamedKeys::new();
    named_keys.insert(OWNER_KEY_NAME.to_string(), owner_key);
    named_keys.insert(ORDERS_KEY_NAME.to_string(), orders_key);
    named_keys.insert(TREASURY_WALLET_KEY_NAME.to_string(), treasury_wallet_key);
    named_keys.insert(FEE_KEY_NAME.to_string(), fee_key);

    let entry_points = entry_points::default();
    let (contract_hash, _version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(String::from(CONTRACT_NAME_KEY_NAME)),
        None,
    );

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key(CONTRACT_NAME_KEY_NAME)
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );

    let constructor_access: URef = storage::create_contract_user_group(
        package_hash,
        CONSTRUCTOR_ENTRY_NAME,
        1,
        Default::default(),
    )
    .unwrap_or_revert()
    .pop()
    .unwrap_or_revert();

    let _: () = runtime::call_contract(contract_hash, CONSTRUCTOR_ENTRY_NAME, runtime_args! {});

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, CONSTRUCTOR_ENTRY_NAME, urefs)
        .unwrap_or_revert();
}
