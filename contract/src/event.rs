use alloc::{collections::BTreeMap, string::ToString, vec::Vec};
use casper_contract::contract_api::storage;
use casper_types::{account::AccountHash, Key, URef, U256, U512};

use crate::constants::{
    ACCEPT_OFFER_ENTRY_NAME, BUY_ORDER_ENTRY_NAME, CANCEL_OFFER_ENTRY_NAME,
    CANCEL_ORDER_ENTRY_NAME, COLLECTION_RUNTIME_ARG_NAME, CREATE_OFFER_ENTRY_NAME,
    CREATE_ORDER_ENTRY_NAME, FEE_RUNTIME_ARG_NAME, MAKER_RUNTIME_ARG_NAME, PRICE_RUNTIME_ARG_NAME,
    SET_FEE_ENTRY_NAME, SET_TREASURY_WALLET_ENTRY_NAME, TOKEN_ID_RUNTIME_ARG_NAME,
    TREASURY_WALLET_RUNTIME_ARG_NAME,
};

pub enum Event {
    OrderCreated {
        maker: AccountHash,
        collection: Key,
        token_id: U256,
        price: U512,
    },
    OrderBought {
        maker: AccountHash,
        collection: Key,
        token_id: U256,
        price: U512,
    },
    OrderCanceled {
        maker: AccountHash,
        collection: Key,
        token_id: U256,
    },
    OfferCreated {
        maker: AccountHash,
        collection: Key,
        token_id: U256,
        price: U512,
    },
    OfferAccepted {
        maker: AccountHash,
        collection: Key,
        token_id: U256,
        price: U512,
    },
    OfferCanceled {
        maker: AccountHash,
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
            maker,
            collection,
            token_id,
            price,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", CREATE_ORDER_ENTRY_NAME.to_string());
            param.insert(MAKER_RUNTIME_ARG_NAME, maker.to_string());
            param.insert(COLLECTION_RUNTIME_ARG_NAME, collection.to_string());
            param.insert(TOKEN_ID_RUNTIME_ARG_NAME, token_id.to_string());
            param.insert(PRICE_RUNTIME_ARG_NAME, price.to_string());
            events.push(param);
        }
        Event::OrderBought {
            maker,
            collection,
            token_id,
            price,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", BUY_ORDER_ENTRY_NAME.to_string());
            param.insert(MAKER_RUNTIME_ARG_NAME, maker.to_string());
            param.insert(COLLECTION_RUNTIME_ARG_NAME, collection.to_string());
            param.insert(TOKEN_ID_RUNTIME_ARG_NAME, token_id.to_string());
            param.insert(PRICE_RUNTIME_ARG_NAME, price.to_string());
            events.push(param);
        }
        Event::OrderCanceled {
            maker,
            collection,
            token_id,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", CANCEL_ORDER_ENTRY_NAME.to_string());
            param.insert(MAKER_RUNTIME_ARG_NAME, maker.to_string());
            param.insert(COLLECTION_RUNTIME_ARG_NAME, collection.to_string());
            param.insert(TOKEN_ID_RUNTIME_ARG_NAME, token_id.to_string());

            events.push(param);
        }
        Event::OfferCreated {
            maker,
            collection,
            token_id,
            price,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", CREATE_OFFER_ENTRY_NAME.to_string());
            param.insert(MAKER_RUNTIME_ARG_NAME, maker.to_string());
            param.insert(COLLECTION_RUNTIME_ARG_NAME, collection.to_string());
            param.insert(TOKEN_ID_RUNTIME_ARG_NAME, token_id.to_string());
            param.insert(PRICE_RUNTIME_ARG_NAME, price.to_string());
            events.push(param);
        }
        Event::OfferAccepted {
            maker,
            collection,
            token_id,
            price,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", ACCEPT_OFFER_ENTRY_NAME.to_string());
            param.insert(MAKER_RUNTIME_ARG_NAME, maker.to_string());
            param.insert(COLLECTION_RUNTIME_ARG_NAME, collection.to_string());
            param.insert(TOKEN_ID_RUNTIME_ARG_NAME, token_id.to_string());
            param.insert(PRICE_RUNTIME_ARG_NAME, price.to_string());
            events.push(param);
        }
        Event::OfferCanceled {
            maker,
            collection,
            token_id,
        } => {
            let mut param = BTreeMap::new();
            param.insert("event_type", CANCEL_OFFER_ENTRY_NAME.to_string());
            param.insert(MAKER_RUNTIME_ARG_NAME, maker.to_string());
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
