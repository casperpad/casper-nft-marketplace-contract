#![no_std]
#![no_main]

extern crate alloc;

use casper_contract::contract_api::runtime;

use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, URef};
const MARKETPLACE_CONTRACT_HASH_ARG_NAME: &str = "marketplace_contract_hash";
const ACCEESS_UREF_KEY_NAME: &str = "casper_nft_marketplace_access";
#[no_mangle]
pub extern "C" fn call() {
    let marketplace_contract_hash: ContractHash = {
        let ido_contract_hash_key: Key = runtime::get_named_arg(MARKETPLACE_CONTRACT_HASH_ARG_NAME);
        ido_contract_hash_key
            .into_hash()
            .map(ContractHash::new)
            .unwrap()
    };
    let access_uref: URef = runtime::call_contract(
        marketplace_contract_hash,
        "get_access_uref",
        runtime_args! {},
    );

    runtime::put_key(ACCEESS_UREF_KEY_NAME, access_uref.into());
}
