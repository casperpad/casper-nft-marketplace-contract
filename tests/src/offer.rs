use alloc::vec::Vec;

use casper_types::{
    account::AccountHash,
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped, ContractHash, U256, U512,
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Bid {
    pub maker: AccountHash,
    pub price: U512,
    pub offer_time: u64,
}

#[derive(Clone, Debug)]
pub struct Offer {
    pub collection: ContractHash,
    pub token_id: U256,
    pub bids: Vec<Bid>,
    pub is_active: bool,
}

impl CLTyped for Offer {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

impl FromBytes for Bid {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (maker, bytes) = AccountHash::from_bytes(bytes)?;
        let (price, bytes) = U512::from_bytes(bytes)?;
        let (offer_time, bytes) = u64::from_bytes(bytes)?;
        let body = Bid {
            maker,
            price,
            offer_time,
        };
        Ok((body, bytes))
    }
}

impl ToBytes for Bid {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend(self.maker.to_bytes()?);
        buffer.extend(self.price.to_bytes()?);
        buffer.extend(self.offer_time.to_bytes()?);
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.maker.serialized_length()
            + self.price.serialized_length()
            + self.offer_time.serialized_length()
    }
}

impl FromBytes for Offer {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (collection, bytes) = ContractHash::from_bytes(bytes)?;
        let (token_id, bytes) = U256::from_bytes(bytes)?;
        let (bids, bytes) = Vec::<Bid>::from_bytes(bytes)?;
        let (is_active, bytes) = bool::from_bytes(bytes)?;

        let body = Offer {
            collection,
            token_id,
            bids,
            is_active,
        };
        Ok((body, bytes))
    }
}

impl ToBytes for Offer {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend(self.collection.to_bytes()?);
        buffer.extend(self.token_id.to_bytes()?);
        buffer.extend(self.bids.to_bytes()?);
        buffer.extend(self.is_active.to_bytes()?);
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.collection.serialized_length()
            + self.token_id.serialized_length()
            + self.bids.serialized_length()
            + self.is_active.serialized_length()
    }
}

impl PartialEq for Offer {
    fn eq(&self, other: &Self) -> bool {
        let eq_collection = self.collection.eq(&other.collection);
        let eq_token_id = self.token_id.eq(&other.token_id);
        let eq_is_active = self.is_active.eq(&other.is_active);

        let matching = self
            .bids
            .iter()
            .zip(&other.bids)
            .filter(|&(a, b)| a == b)
            .count();
        let eq_bids = matching == self.bids.len();

        eq_collection && eq_token_id && eq_is_active && eq_bids
    }
}
