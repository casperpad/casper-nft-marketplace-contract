#![no_std]
#![no_main]

use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, URef, U256, U512};

const MARKETPLACE_CONTRACT_HASH_ARG_NAME: &str = "marketplace_contract_hash";
const CREATE_OFFER_ENTRY_NAME: &str = "create_offer";
const GET_PURSE_ENTRY_NAME: &str = "get_purse";
const COLLECTION_RUNTIME_ARG_NAME: &str = "collection";
const AMOUNT_RUNTIME_ARG_NAME: &str = "amount";
const TOKEN_ID_RUNTIME_ARG_NAME: &str = "token_id";

#[no_mangle]
fn call() {
    let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let collection_key: Key = runtime::get_named_arg(COLLECTION_RUNTIME_ARG_NAME);
    let token_id: U256 = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);

    let marketplace_contract_hash: ContractHash = {
        let ido_contract_hash_key: Key = runtime::get_named_arg(MARKETPLACE_CONTRACT_HASH_ARG_NAME);
        ido_contract_hash_key
            .into_hash()
            .map(ContractHash::new)
            .unwrap()
    };

    let sender_purse: URef = account::get_main_purse();

    let deposit_purse: URef = runtime::call_contract(
        marketplace_contract_hash,
        GET_PURSE_ENTRY_NAME,
        runtime_args! {},
    );

    system::transfer_from_purse_to_purse(sender_purse, deposit_purse, amount, None)
        .unwrap_or_revert();

    runtime::call_contract::<()>(
        marketplace_contract_hash,
        CREATE_OFFER_ENTRY_NAME,
        runtime_args! {
            COLLECTION_RUNTIME_ARG_NAME => collection_key,
            TOKEN_ID_RUNTIME_ARG_NAME => token_id,
            AMOUNT_RUNTIME_ARG_NAME => amount
        },
    );
}
