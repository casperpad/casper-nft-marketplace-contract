#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{
    collections::BTreeSet,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, contracts::NamedKeys, runtime_args, CLValue, ContractHash, Key,
    RuntimeArgs, URef, U256, U512,
};
use constants::{
    ADMINS_GROUP_NAME, ADMINS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, BID_ID_RUNTIME_ARG_NAME,
    COLLECTION_RUNTIME_ARG_NAME, CONSTRUCTOR_ENTRY_NAME, CONTRACT_NAME_KEY_NAME, FEE_KEY_NAME,
    FEE_RUNTIME_ARG_NAME, ON_ORDERS_KEY_NAME, ORDERS_KEY_NAME, OWNER_KEY_NAME,
    OWNER_RUNTIME_ARG_NAME, PRICE_RUNTIME_ARG_NAME, PURSE_BALANCE_KEY_NAME,
    TOKEN_ID_RUNTIME_ARG_NAME, TREASURY_WALLET_KEY_NAME,
};
use detail::store_result;
use error::Error;
use icep47::ICEP47;
use offer::{Bid, Offer};
use on_orders::OnOrder;
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
mod on_orders;
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
pub extern "C" fn get_access_uref() {
    let account_hash_str = runtime::get_caller().to_string();

    let account_hash_uref = match runtime::get_key(&account_hash_str) {
        Some(uref) => uref.into_uref().unwrap(),
        None => runtime::revert(Error::InvalidContext),
    };

    let return_value = CLValue::from_t(account_hash_uref).unwrap_or_revert();
    runtime::ret(return_value)
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

    let mut on_orders = on_orders::read_on_orders();

    let find_result = on_orders::find(collection, token_id);

    if find_result != None {
        runtime::revert(Error::OrderExist);
    }
    on_orders.push((collection, token_id));
    on_orders::write_on_orders(on_orders);

    let maker = runtime::get_caller();

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

    let is_active = true;
    let order = Order {
        collection,
        token_id,
        maker,
        price,
        is_active,
    };
    orders::write_order(order);
}

#[no_mangle]
pub extern "C" fn change_order_price() {}

#[no_mangle]
pub extern "C" fn cancel_order() {
    let collection: ContractHash = {
        let collection_key: Key = runtime::get_named_arg(COLLECTION_RUNTIME_ARG_NAME);
        ContractHash::new(collection_key.into_hash().unwrap())
    };
    let token_id: U256 = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let find_result = on_orders::find(collection, token_id);
    if find_result == None {
        runtime::revert(Error::OrderNotExist);
    }
    let mut order = orders::read_order(collection, token_id);
    let caller = runtime::get_caller();
    if caller != order.maker {
        runtime::revert(Error::NotOrderMaker);
    }

    // Refund the token
    ICEP47::new(order.collection).transfer(Key::from(order.maker), vec![order.token_id]);
    order.is_active = false;
    orders::write_order(order);

    let mut on_orders: Vec<OnOrder> = on_orders::read_on_orders();
    on_orders.remove(find_result.unwrap());
    on_orders::write_on_orders(on_orders);

    store_result(order);
}

#[no_mangle]
pub extern "C" fn buy_order() {
    let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let collection: ContractHash = {
        let collection_key: Key = runtime::get_named_arg(COLLECTION_RUNTIME_ARG_NAME);
        ContractHash::new(collection_key.into_hash().unwrap())
    };
    let token_id: U256 = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);

    let find_result = on_orders::find(collection, token_id);
    if find_result == None {
        runtime::revert(Error::OrderNotExist);
    }

    let _ = purse::checked_balance();
    let mut order = orders::read_order(collection, token_id);
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
    purse::transfer_with_fee(order.maker, order.price);

    order.is_active = false;

    let mut on_orders: Vec<OnOrder> = on_orders::read_on_orders();
    on_orders.remove(find_result.unwrap());
    on_orders::write_on_orders(on_orders);
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
    let bid_id: usize = {
        let id: u8 = runtime::get_named_arg(BID_ID_RUNTIME_ARG_NAME);
        usize::from(id)
    };
    let collection: ContractHash = {
        let collection_key: Key = runtime::get_named_arg(COLLECTION_RUNTIME_ARG_NAME);
        ContractHash::new(collection_key.into_hash().unwrap())
    };
    let token_id: U256 = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let caller = runtime::get_caller();
    let token_owner = ICEP47::new(collection).owner_of(token_id).unwrap();
    if token_owner != Key::from(caller) {
        runtime::revert(Error::PermissionDenied);
    }
    let mut offer = offers::read_offer(collection, token_id).unwrap_or_revert();
    let accepted_bid = &offer.bids.get(bid_id).unwrap().clone();
    // Send cspr to token owner and transfer nft to bidder
    purse::transfer_with_fee(caller, accepted_bid.price);
    ICEP47::new(collection).transfer_from(
        Key::from(caller),
        Key::from(accepted_bid.maker),
        vec![token_id],
    );
    // for other bidders, refund
    for bid in &offer.bids {
        if !bid.eq(accepted_bid) {
            purse::transfer(bid.maker, bid.price);
        }
    }
    offer.is_active = false;

    store_result(ICEP47::new(collection).owner_of(token_id));
    offers::write_offer(offer);
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
    let (contract_package_hash, _access_uref) = storage::create_contract_package_at_hash();
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
        let fee = U512::from(25u8);
        let fee_uref: URef = storage::new_uref(fee).into_read_write();
        Key::from(fee_uref)
    };

    let orders_key: Key = {
        let uref = storage::new_dictionary(ORDERS_KEY_NAME).unwrap();
        Key::from(uref)
    };

    let on_orders_key: Key = {
        let init_value: Vec<OnOrder> = Vec::new();
        let uref: URef = storage::new_uref(init_value).into_read_write();
        Key::from(uref)
    };

    let admins: Vec<AccountHash> = runtime::get_named_arg(ADMINS_RUNTIME_ARG_NAME);

    let mut named_keys = NamedKeys::new();
    named_keys.insert(OWNER_KEY_NAME.to_string(), owner_key);
    named_keys.insert(ORDERS_KEY_NAME.to_string(), orders_key);
    named_keys.insert(TREASURY_WALLET_KEY_NAME.to_string(), treasury_wallet_key);
    named_keys.insert(FEE_KEY_NAME.to_string(), fee_key);
    named_keys.insert(ON_ORDERS_KEY_NAME.to_string(), on_orders_key);

    let mut admin_group = storage::create_contract_user_group(
        contract_package_hash,
        ADMINS_GROUP_NAME,
        (admins.len() + 1) as u8,
        Default::default(),
    )
    .unwrap();

    named_keys.insert(
        runtime::get_caller().to_string(),
        admin_group.pop().unwrap().into(),
    );

    for (i, uref) in admin_group.into_iter().enumerate() {
        named_keys.insert(admins[i].to_string(), uref.into());
    }

    let entry_points = entry_points::default();

    let constructor_access: URef = storage::create_contract_user_group(
        contract_package_hash,
        CONSTRUCTOR_ENTRY_NAME,
        1,
        Default::default(),
    )
    .unwrap_or_revert()
    .pop()
    .unwrap_or_revert();

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    runtime::put_key(CONTRACT_NAME_KEY_NAME, contract_package_hash.into());
    runtime::call_contract::<()>(contract_hash, CONSTRUCTOR_ENTRY_NAME, runtime_args! {});

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(contract_package_hash, CONSTRUCTOR_ENTRY_NAME, urefs)
        .unwrap_or_revert();
}
