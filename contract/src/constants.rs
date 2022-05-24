// KEY NAMES
pub const CONTRACT_NAME_KEY_NAME: &str = "casper_nft_marketplace";
pub const TREASURY_WALLET_KEY_NAME: &str = "treasury_wallet";
pub const OWNER_KEY_NAME: &str = "owner";
pub const ORDERS_KEY_NAME: &str = "orders";
pub const RESULT_KEY_NAME: &str = "result";
pub const PURSE_KEY_NAME: &str = "purse";
pub const PURSE_BALANCE_KEY_NAME: &str = "purse_balance";
pub const FEE_KEY_NAME: &str = "fee";

// RUNTIME ARG NAMES
pub const OWNER_RUNTIME_ARG_NAME: &str = "owner";
pub const COLLECTION_RUNTIME_ARG_NAME: &str = "collection";
pub const TOKEN_ID_RUNTIME_ARG_NAME: &str = "token_id";
pub const PRICE_RUNTIME_ARG_NAME: &str = "price";
pub const ORDER_ID_RUNTIME_ARG_NAME: &str = "order_id";
pub const BID_ID_RUNTIME_ARG_NAME: &str = "bid_id";
pub const FEE_RUNTIME_ARG_NAME: &str = "fee";
pub const AMOUNT_RUNTIME_ARG_NAME: &str = "amount";
// ENTRY POINT NAMES
pub const TRANSFER_OWNERSHIP_ENTRY_NAME: &str = "transfer_ownership";
pub const SET_TREASURY_WALLET_ENTRY_NAME: &str = "set_treasury_wallet";
pub const CREATE_ORDER_ENTRY_NAME: &str = "create_order";
pub const CANCEL_ORDER_ENTRY_NAME: &str = "cancel_order";
pub const BUY_ORDER_ENTRY_NAME: &str = "buy_order";
pub const CREATE_OFFER_ENTRY_NAME: &str = "create_offer";
pub const CANCEL_OFFER_ENTRY_NAME: &str = "cancel_offer";
pub const ACCEPT_OFFER_ENTRY_NAME: &str = "accept_offer";
pub const CONSTRUCTOR_ENTRY_NAME: &str = "constructor";
pub const GET_PURSE_ENTRY_NAME: &str = "get_purse";