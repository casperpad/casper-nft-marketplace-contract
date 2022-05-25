extern crate alloc;

mod offer;
mod order;

mod meta {
    use std::collections::BTreeMap;

    type Meta = BTreeMap<String, String>;
    pub fn contract_meta() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("origin".to_string(), "fire".to_string());
        meta
    }

    pub fn red_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "red".to_string());
        meta
    }

    pub fn blue_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "blue".to_string());
        meta
    }

    pub fn black_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "black".to_string());
        meta
    }

    pub fn gold_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "gold".to_string());
        meta
    }
}

#[cfg(test)]
mod tests {

    use casper_engine_test_support::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        DEFAULT_PAYMENT, DEFAULT_RUN_GENESIS_REQUEST,
    };
    use casper_execution_engine::core::engine_state::ExecuteRequest;
    use casper_types::{
        account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractHash,
        ContractPackageHash, Key, PublicKey, RuntimeArgs, SecretKey, U256, U512,
    };

    use crate::{meta, offer::Offer, order::Order};

    // KEY NAMES
    const CONTRACT_NAME_KEY_NAME: &str = "casper_nft_marketplace";
    const RESULT_KEY_NAME: &str = "result";

    // ENTRY
    const CREATE_ORDER_ENTRY_NAME: &str = "create_order";
    const CANCEL_ORDER_ENTRY_NAME: &str = "cancel_order";
    const BUY_ORDER_ENTRY_NAME: &str = "buy_order";
    const TRANSFER_OWNERSHIP_ENTRY_NAME: &str = "transfer_ownership";
    // RUNTIME ARG
    const COLLECTION_RUNTIME_ARG_NAME: &str = "collection";
    const TOKEN_ID_RUNTIME_ARG_NAME: &str = "token_id";
    const PRICE_RUNTIME_ARG_NAME: &str = "price";
    const ORDER_ID_RUNTIME_ARG_NAME: &str = "order_id";
    const OWNER_ARG_NAME: &str = "owner";
    const AMOUNT_RUNTIME_ARG_NAME: &str = "amount";
    const MARKETPLACE_CONTRACT_HASH_ARG_NAME: &str = "marketplace_contract_hash";
    const BID_ID_RUNTIME_ARG_NAME: &str = "bid_id";

    const CONTRACT_WASM: &str = "contract.wasm";
    const PRE_BUY_ORDER_CONTRACT_WASM: &str = "pre_buy_order.wasm";
    const PER_CREATE_OFFER_CONTRACT_WASM: &str = "pre_create_offer.wasm";
    const CEP47_CONTRACT_WASM: &str = "cep47-token.wasm";
    const AUTHORIZE_ACCOUNT_CONTRACT_WASM: &str = "authorize_account.wasm";

    const NFT_NAME: &str = "DragonsNFT";
    const NFT_SYMBOL: &str = "DGNFT";

    #[derive(Copy, Clone)]
    struct TestContext {
        marketplace_contract_package: ContractPackageHash,
        marketplace_contract: ContractHash,
        nft_contract_hash: ContractHash,
    }

    fn fund_account(account: &AccountHash) -> ExecuteRequest {
        let deploy_item = DeployItemBuilder::new()
            .with_address(*DEFAULT_ACCOUNT_ADDR)
            .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
            .with_empty_payment_bytes(runtime_args! {"amount" => *DEFAULT_PAYMENT})
            .with_transfer_args(runtime_args! {
                "amount" => U512::from(30_000_000_000_000_u64),
                "target" => *account,
                "id" => <Option::<u64>>::None
            })
            .with_deploy_hash([1; 32])
            .build();

        ExecuteRequestBuilder::from_deploy_item(deploy_item).build()
    }

    fn setup() -> (InMemoryWasmTestBuilder, TestContext) {
        let mut builder = InMemoryWasmTestBuilder::default();

        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

        let install_contract = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CONTRACT_WASM,
            runtime_args! {
                "admins" => vec![*DEFAULT_ACCOUNT_ADDR,account(0),account(1)]
            },
        )
        .build();

        builder.exec(install_contract).expect_success().commit();

        let install_cep47_contract = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CEP47_CONTRACT_WASM,
            runtime_args! {
                "contract_name" => NFT_NAME,
                "name" => NFT_NAME,
                "symbol" => NFT_SYMBOL,
                "meta" => meta::contract_meta()
            },
        )
        .build();

        builder
            .exec(install_cep47_contract)
            .expect_success()
            .commit();

        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("should have account");

        let marketplace_contract_package = account
            .named_keys()
            .get(CONTRACT_NAME_KEY_NAME)
            .and_then(|key| key.into_hash())
            .map(ContractPackageHash::new)
            .expect("should have contract package hash");
        let marketplace_contract =
            get_latest_contract_hash(&mut builder, marketplace_contract_package);

        let nft_contract_hash = account
            .named_keys()
            .get(&format!("{}_contract_hash", NFT_NAME))
            .and_then(|key| key.into_hash())
            .map(ContractHash::new)
            .expect("should have contract hash");

        let mut accounts = Vec::new();
        for i in 0..10u8 {
            let secret_key: SecretKey = SecretKey::ed25519_from_bytes([i; 32]).unwrap();
            let public_key: PublicKey = (&secret_key).into();
            let account_hash = AccountHash::from(&public_key);
            accounts.push(account_hash);
            builder
                .exec(fund_account(&account_hash))
                .expect_success()
                .commit();
        }
        let test_context = TestContext {
            marketplace_contract_package,
            marketplace_contract,
            nft_contract_hash,
        };
        (builder, test_context)
    }

    fn account(index: u8) -> AccountHash {
        let secret_key: SecretKey = SecretKey::ed25519_from_bytes([index; 32]).unwrap();
        let public_key: PublicKey = (&secret_key).into();
        AccountHash::from(&public_key)
    }

    fn get_latest_contract_hash(
        builder: &mut InMemoryWasmTestBuilder,
        contract_package_hash: ContractPackageHash,
    ) -> ContractHash {
        let contract_package = builder
            .get_contract_package(contract_package_hash)
            .expect("should have contract package");
        let enabled_versions = contract_package.enabled_versions();
        let (_version, contract_hash) = enabled_versions
            .iter()
            .rev()
            .next()
            .expect("should have latest version");
        *contract_hash
    }

    fn get_test_result<T: FromBytes + CLTyped>(
        builder: &mut InMemoryWasmTestBuilder,
        contract_hash: ContractHash,
    ) -> T {
        builder.get_value(contract_hash, RESULT_KEY_NAME)
    }

    fn call_contract(
        builder: &mut InMemoryWasmTestBuilder,
        contract_hash: ContractHash,
        sender: AccountHash,
        entry_point: &str,
        session_args: RuntimeArgs,
    ) {
        let exec_request = ExecuteRequestBuilder::contract_call_by_hash(
            sender,
            contract_hash,
            entry_point,
            session_args,
        )
        .build();
        builder.exec(exec_request).expect_success().commit();
    }

    fn authorize_account(
        builder: &mut InMemoryWasmTestBuilder,
        context: TestContext,
        account: AccountHash,
    ) {
        let install_contract = ExecuteRequestBuilder::standard(
            account,
            AUTHORIZE_ACCOUNT_CONTRACT_WASM,
            runtime_args! {
                MARKETPLACE_CONTRACT_HASH_ARG_NAME => Key::from(context.marketplace_contract)
            },
        )
        .build();

        builder.exec(install_contract).expect_success().commit();
    }

    fn set_treasury_wallet(
        builder: &mut InMemoryWasmTestBuilder,
        context: TestContext,
        caller: AccountHash,
        treasury_wallet: String,
    ) {
        call_contract(
            builder,
            context.marketplace_contract,
            caller,
            "set_treasury_wallet",
            runtime_args! {
                "treasury_wallet" => treasury_wallet,
            },
        );
    }

    fn transfer_ownership(builder: &mut InMemoryWasmTestBuilder, context: TestContext) {
        call_contract(
            builder,
            context.marketplace_contract,
            *DEFAULT_ACCOUNT_ADDR,
            TRANSFER_OWNERSHIP_ENTRY_NAME,
            runtime_args! {
                OWNER_ARG_NAME => Key::from(account(0)),
            },
        );
    }

    fn create_order(builder: &mut InMemoryWasmTestBuilder, context: TestContext) {
        call_contract(
            builder,
            context.marketplace_contract,
            *DEFAULT_ACCOUNT_ADDR,
            CREATE_ORDER_ENTRY_NAME,
            runtime_args! {
                COLLECTION_RUNTIME_ARG_NAME => Key::from(context.nft_contract_hash),
                TOKEN_ID_RUNTIME_ARG_NAME => U256::from(0),
                PRICE_RUNTIME_ARG_NAME => U512::from(1000).checked_mul(U512::exp10(9)).unwrap(),
            },
        );
    }

    fn cancel_order(builder: &mut InMemoryWasmTestBuilder, context: TestContext) {
        call_contract(
            builder,
            context.marketplace_contract,
            *DEFAULT_ACCOUNT_ADDR,
            CANCEL_ORDER_ENTRY_NAME,
            runtime_args! {
                ORDER_ID_RUNTIME_ARG_NAME => U256::from(0)
            },
        );
    }

    fn pre_buy_order(builder: &mut InMemoryWasmTestBuilder, context: TestContext) {
        let install_pre_buy_order_contract = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            PRE_BUY_ORDER_CONTRACT_WASM,
            runtime_args! {
                ORDER_ID_RUNTIME_ARG_NAME => U256::from(0),
                AMOUNT_RUNTIME_ARG_NAME => U512::from(1000).checked_mul(U512::exp10(9)).unwrap(),
                MARKETPLACE_CONTRACT_HASH_ARG_NAME => Key::from(context.marketplace_contract)
            },
        )
        .build();

        builder
            .exec(install_pre_buy_order_contract)
            .expect_success()
            .commit();
    }

    fn _buy_order(builder: &mut InMemoryWasmTestBuilder, context: TestContext) {
        call_contract(
            builder,
            context.marketplace_contract,
            *DEFAULT_ACCOUNT_ADDR,
            BUY_ORDER_ENTRY_NAME,
            runtime_args! {
                ORDER_ID_RUNTIME_ARG_NAME => U256::from(0),
                AMOUNT_RUNTIME_ARG_NAME => U512::from(1000)
            },
        );
    }

    fn mint_nft(builder: &mut InMemoryWasmTestBuilder, context: TestContext) {
        call_contract(
            builder,
            context.nft_contract_hash,
            *DEFAULT_ACCOUNT_ADDR,
            "mint",
            runtime_args! {
                "recipient" => Key::from(*DEFAULT_ACCOUNT_ADDR),
                "token_ids" => vec![U256::from(0),U256::from(1)],
                "token_metas" => vec![meta::red_dragon(),meta::black_dragon()]
            },
        );
    }

    fn approve_nft(builder: &mut InMemoryWasmTestBuilder, context: TestContext) {
        call_contract(
            builder,
            context.nft_contract_hash,
            *DEFAULT_ACCOUNT_ADDR,
            "approve",
            runtime_args! {
                "spender" => Key::from(context.marketplace_contract_package),
                "token_ids" => vec![U256::from(0)],
            },
        );
    }

    fn _transfer_nft(builder: &mut InMemoryWasmTestBuilder, context: TestContext) {
        call_contract(
            builder,
            context.nft_contract_hash,
            *DEFAULT_ACCOUNT_ADDR,
            "transfer",
            runtime_args! {
                "recipient" => Key::from(context.marketplace_contract_package),
                "token_ids" => vec![U256::from(0)],
            },
        );
    }

    fn pre_create_offer(
        builder: &mut InMemoryWasmTestBuilder,
        context: TestContext,
        maker: AccountHash,
        token_id: U256,
        price: U512,
    ) {
        let install_pre_create_offer_contract = ExecuteRequestBuilder::standard(
            maker,
            PER_CREATE_OFFER_CONTRACT_WASM,
            runtime_args! {
                COLLECTION_RUNTIME_ARG_NAME => Key::from(context.nft_contract_hash),
                TOKEN_ID_RUNTIME_ARG_NAME => token_id,
                AMOUNT_RUNTIME_ARG_NAME => price,
                MARKETPLACE_CONTRACT_HASH_ARG_NAME => Key::from(context.marketplace_contract)
            },
        )
        .build();

        builder
            .exec(install_pre_create_offer_contract)
            .expect_success()
            .commit();
    }

    fn _create_offer(
        builder: &mut InMemoryWasmTestBuilder,
        context: TestContext,
        maker: AccountHash,
        token_id: U256,
        price: U512,
    ) {
        call_contract(
            builder,
            context.marketplace_contract,
            maker,
            "create_offer",
            runtime_args! {
                COLLECTION_RUNTIME_ARG_NAME => Key::from(context.nft_contract_hash),
                TOKEN_ID_RUNTIME_ARG_NAME => token_id,
                AMOUNT_RUNTIME_ARG_NAME => price
            },
        );
    }

    fn cancel_offer(
        builder: &mut InMemoryWasmTestBuilder,
        context: TestContext,
        maker: AccountHash,
        token_id: U256,
    ) {
        call_contract(
            builder,
            context.marketplace_contract,
            maker,
            "cancel_offer",
            runtime_args! {
                COLLECTION_RUNTIME_ARG_NAME => Key::from(context.nft_contract_hash),
                TOKEN_ID_RUNTIME_ARG_NAME => token_id,

            },
        );
    }

    fn accept_offer(
        builder: &mut InMemoryWasmTestBuilder,
        context: TestContext,

        token_id: U256,
        bid_id: u8,
    ) {
        call_contract(
            builder,
            context.marketplace_contract,
            *DEFAULT_ACCOUNT_ADDR,
            "accept_offer",
            runtime_args! {
                COLLECTION_RUNTIME_ARG_NAME => Key::from(context.nft_contract_hash),
                TOKEN_ID_RUNTIME_ARG_NAME => token_id,
                BID_ID_RUNTIME_ARG_NAME => bid_id
            },
        );
    }

    #[test]
    fn should_set_treasury_wallet() {
        let (mut builder, context) = setup();
        let admin = *DEFAULT_ACCOUNT_ADDR;
        let treasury_wallet = account(1);
        authorize_account(&mut builder, context, admin);
        set_treasury_wallet(&mut builder, context, admin, treasury_wallet.to_string());
    }

    #[test]
    fn should_create_offer() {
        let (mut builder, context) = setup();
        pre_create_offer(
            &mut builder,
            context,
            *DEFAULT_ACCOUNT_ADDR,
            U256::zero(),
            U512::from(2).checked_mul(U512::exp10(9)).unwrap(),
        );

        pre_create_offer(
            &mut builder,
            context,
            account(2),
            U256::zero(),
            U512::from(3).checked_mul(U512::exp10(9)).unwrap(),
        );
        let offer: Offer = get_test_result(&mut builder, context.marketplace_contract);
        println!("{:?}", offer);
        assert!(offer.bids.len() == 2);
    }

    #[test]
    fn should_cancel_offer() {
        let (mut builder, context) = setup();
        pre_create_offer(
            &mut builder,
            context,
            *DEFAULT_ACCOUNT_ADDR,
            U256::zero(),
            U512::from(2).checked_mul(U512::exp10(9)).unwrap(),
        );

        pre_create_offer(
            &mut builder,
            context,
            account(2),
            U256::zero(),
            U512::from(3).checked_mul(U512::exp10(9)).unwrap(),
        );

        cancel_offer(&mut builder, context, account(2), U256::zero());

        let offer: Offer = get_test_result(&mut builder, context.marketplace_contract);
        println!("{:?}", offer);
        assert!(offer.bids.len() == 1);
    }

    #[test]
    fn should_accept_offer() {
        let (mut builder, context) = setup();
        pre_create_offer(
            &mut builder,
            context,
            *DEFAULT_ACCOUNT_ADDR,
            U256::zero(),
            U512::from(2).checked_mul(U512::exp10(9)).unwrap(),
        );

        pre_create_offer(
            &mut builder,
            context,
            account(2),
            U256::zero(),
            U512::from(3).checked_mul(U512::exp10(9)).unwrap(),
        );
        mint_nft(&mut builder, context);
        approve_nft(&mut builder, context);
        accept_offer(&mut builder, context, U256::zero(), 1u8);
        let owner: Option<Key> = get_test_result(&mut builder, context.marketplace_contract);
        assert!(owner == Some(Key::from(account(2))));
    }

    #[test]
    fn should_install_contract() {
        let _ = setup();
    }

    #[test]
    fn should_transfer_ownership() {
        let (mut builder, context) = setup();
        transfer_ownership(&mut builder, context);
    }

    #[test]
    fn should_mint_nft() {
        let (mut builder, context) = setup();
        mint_nft(&mut builder, context);
    }

    #[test]
    fn should_approve_nft() {
        let (mut builder, context) = setup();
        mint_nft(&mut builder, context);

        approve_nft(&mut builder, context);
    }

    #[test]
    fn should_transfer_from_nft() {
        let (mut builder, context) = setup();
        mint_nft(&mut builder, context);
        approve_nft(&mut builder, context);
    }

    #[test]
    fn should_create_order() {
        let (mut builder, context) = setup();

        mint_nft(&mut builder, context);

        approve_nft(&mut builder, context);

        create_order(&mut builder, context);
    }

    #[test]
    fn should_buy_order() {
        let (mut builder, context) = setup();

        mint_nft(&mut builder, context);

        approve_nft(&mut builder, context);

        create_order(&mut builder, context);
        pre_buy_order(&mut builder, context);
    }

    #[test]
    fn should_cancel_order() {
        let (mut builder, context) = setup();

        mint_nft(&mut builder, context);

        approve_nft(&mut builder, context);

        create_order(&mut builder, context);

        cancel_order(&mut builder, context);

        let order: Order = get_test_result(&mut builder, context.marketplace_contract);
        println!("{:?}", order);
        assert_eq!(order.is_active, false);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
