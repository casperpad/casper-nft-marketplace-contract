use alloc::vec::Vec;
use casper_types::{
    account::AccountHash,
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped, U512,
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Bid {
    pub maker: AccountHash,
    pub price: U512,
    pub offer_time: u64,
}

impl CLTyped for Bid {
    fn cl_type() -> casper_types::CLType {
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
