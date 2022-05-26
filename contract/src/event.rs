use alloc::{collections::BTreeMap, string::ToString, vec::Vec};
use casper_contract::contract_api::storage;
use casper_types::{account::AccountHash, Key, URef, U256, U512};

use crate::constants::{
    ACCEPT_OFFER_ENTRY_NAME, BUY_ORDER_ENTRY_NAME, CANCEL_OFFER_ENTRY_NAME,
    CANCEL_ORDER_ENTRY_NAME, COLLECTION_RUNTIME_ARG_NAME, CREATE_OFFER_ENTRY_NAME,
    CREATE_ORDER_ENTRY_NAME, FEE_RUNTIME_ARG_NAME, OFFERER_RUNTIME_ARG_NAME,
    PRICE_RUNTIME_ARG_NAME, SET_FEE_ENTRY_NAME, SET_TREASURY_WALLET_ENTRY_NAME,
    TOKEN_ID_RUNTIME_ARG_NAME, TREASURY_WALLET_RUNTIME_ARG_NAME,
};

pub enum Event {
    OrderCreated {
        offerer: AccountHash,
        collection: Key,
        token_id: U256,
        price: U512,
    },
    OrderBought {
        offerer: AccountHash,
        collection: Key,
        token_id: U256,
        price: U512,
    },
    OrderCanceled {
        offerer: AccountHash,
        collection: Key,
        token_id: U256,
    },
    OfferCreated {
        offerer: AccountHash,
        collection: Key,
        token_id: U256,
        price: U512,
    },
    OfferAccepted {
        offerer: AccountHash,
        collection: Key,
        token_id: U256,
        price: U512,
    },
    OfferCanceled {
        offerer: AccountHash,
        collection: Key,
        token_id: U256,
    },
    TreasuryWalletChanged {
        treasury_wallet: AccountHash,
    },
    FeeChanged {
        fee: U512,
    },
}

pub(crate) fn emit(event: &Event) {
    let mut events = Vec::new();
    match event {
        Event::OrderCreated {
            offerer,
            collection,
            token_id,
            price,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", CREATE_ORDER_ENTRY_NAME.to_string());
            param.insert(OFFERER_RUNTIME_ARG_NAME, offerer.to_string());
            param.insert(COLLECTION_RUNTIME_ARG_NAME, collection.to_string());
            param.insert(TOKEN_ID_RUNTIME_ARG_NAME, token_id.to_string());
            param.insert(PRICE_RUNTIME_ARG_NAME, price.to_string());
            events.push(param);
        }
        Event::OrderBought {
            offerer,
            collection,
            token_id,
            price,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", BUY_ORDER_ENTRY_NAME.to_string());
            param.insert(OFFERER_RUNTIME_ARG_NAME, offerer.to_string());
            param.insert(COLLECTION_RUNTIME_ARG_NAME, collection.to_string());
            param.insert(TOKEN_ID_RUNTIME_ARG_NAME, token_id.to_string());
            param.insert(PRICE_RUNTIME_ARG_NAME, price.to_string());
            events.push(param);
        }
        Event::OrderCanceled {
            offerer,
            collection,
            token_id,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", CANCEL_ORDER_ENTRY_NAME.to_string());
            param.insert(OFFERER_RUNTIME_ARG_NAME, offerer.to_string());
            param.insert(COLLECTION_RUNTIME_ARG_NAME, collection.to_string());
            param.insert(TOKEN_ID_RUNTIME_ARG_NAME, token_id.to_string());

            events.push(param);
        }
        Event::OfferCreated {
            offerer,
            collection,
            token_id,
            price,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", CREATE_OFFER_ENTRY_NAME.to_string());
            param.insert(OFFERER_RUNTIME_ARG_NAME, offerer.to_string());
            param.insert(COLLECTION_RUNTIME_ARG_NAME, collection.to_string());
            param.insert(TOKEN_ID_RUNTIME_ARG_NAME, token_id.to_string());
            param.insert(PRICE_RUNTIME_ARG_NAME, price.to_string());
            events.push(param);
        }
        Event::OfferAccepted {
            offerer,
            collection,
            token_id,
            price,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", ACCEPT_OFFER_ENTRY_NAME.to_string());
            param.insert(OFFERER_RUNTIME_ARG_NAME, offerer.to_string());
            param.insert(COLLECTION_RUNTIME_ARG_NAME, collection.to_string());
            param.insert(TOKEN_ID_RUNTIME_ARG_NAME, token_id.to_string());
            param.insert(PRICE_RUNTIME_ARG_NAME, price.to_string());
            events.push(param);
        }
        Event::OfferCanceled {
            offerer,
            collection,
            token_id,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", CANCEL_OFFER_ENTRY_NAME.to_string());
            param.insert(OFFERER_RUNTIME_ARG_NAME, offerer.to_string());
            param.insert(COLLECTION_RUNTIME_ARG_NAME, collection.to_string());
            param.insert(TOKEN_ID_RUNTIME_ARG_NAME, token_id.to_string());

            events.push(param);
        }
        Event::TreasuryWalletChanged { treasury_wallet } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", SET_TREASURY_WALLET_ENTRY_NAME.to_string());
            param.insert(
                TREASURY_WALLET_RUNTIME_ARG_NAME,
                treasury_wallet.to_string(),
            );
            events.push(param);
        }
        Event::FeeChanged { fee } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", SET_FEE_ENTRY_NAME.to_string());
            param.insert(FEE_RUNTIME_ARG_NAME, fee.to_string());
            events.push(param);
        }
    }
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
