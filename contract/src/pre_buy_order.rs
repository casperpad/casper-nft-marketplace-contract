#![no_std]
#![no_main]

use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, URef, U256, U512};

const MARKETPLACE_CONTRACT_HASH_ARG_NAME: &str = "marketplace_contract_hash";
const BUY_ORDER_ENTRY_NAME: &str = "buy_order";
const GET_PURSE_ENTRY_NAME: &str = "get_purse";
const AMOUNT_RUNTIME_ARG_NAME: &str = "amount";
const ORDER_ID_ARG_NAME: &str = "order_id";

#[no_mangle]
fn call() {
    let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let order_id: U256 = runtime::get_named_arg(ORDER_ID_ARG_NAME);

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
        BUY_ORDER_ENTRY_NAME,
        runtime_args! {
            ORDER_ID_ARG_NAME => order_id,
            AMOUNT_RUNTIME_ARG_NAME => amount,
        },
    );
}
