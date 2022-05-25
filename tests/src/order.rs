use alloc::vec::Vec;
use casper_types::{
    account::AccountHash,
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped, ContractHash, U256, U512,
};

#[derive(Clone, Copy, Debug)]
pub struct Order {
    pub collection: ContractHash,
    pub token_id: U256,
    pub maker: AccountHash,
    pub price: U512,
    pub is_active: bool,
}

impl CLTyped for Order {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

impl ToBytes for Order {
    #[inline(always)]
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut result: Vec<u8> = Vec::new();

        result.append(&mut self.collection.into_bytes().unwrap());
        result.append(&mut self.token_id.into_bytes().unwrap());
        result.append(&mut self.maker.into_bytes().unwrap());
        result.append(&mut self.price.into_bytes().unwrap());
        result.append(&mut self.is_active.into_bytes().unwrap());
        Ok(result)
    }

    #[inline(always)]
    fn serialized_length(&self) -> usize {
        self.collection.serialized_length()
            + self.token_id.serialized_length()
            + self.maker.serialized_length()
            + self.price.serialized_length()
            + self.price.serialized_length()
    }

    fn into_bytes(self) -> Result<Vec<u8>, casper_types::bytesrepr::Error>
    where
        Self: Sized,
    {
        self.to_bytes()
    }
}

impl FromBytes for Order {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (collection, bytes) = ContractHash::from_bytes(bytes).unwrap();
        let (token_id, bytes) = U256::from_bytes(bytes).unwrap();
        let (maker, bytes) = AccountHash::from_bytes(bytes).unwrap();
        let (price, bytes) = U512::from_bytes(bytes).unwrap();
        let (is_active, bytes) = bool::from_bytes(bytes).unwrap();

        Ok((
            Order {
                collection,
                token_id,
                maker,
                price,
                is_active,
            },
            bytes,
        ))
    }
}
