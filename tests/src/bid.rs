use alloc::vec::Vec;
use casper_types::{
    account::AccountHash,
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped, U512,
};

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum BidStatus {
    Pending = 0,
    Accepted,
    NotAccepted,
    Canceled,
}

impl CLTyped for BidStatus {
    fn cl_type() -> CLType {
        CLType::U8
    }
}

impl ToBytes for BidStatus {
    fn to_bytes(&self) -> Result<alloc::vec::Vec<u8>, casper_types::bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend((*self as u8).to_bytes()?);
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        (*self as u8).serialized_length()
    }

    fn into_bytes(self) -> Result<alloc::vec::Vec<u8>, bytesrepr::Error>
    where
        Self: Sized,
    {
        self.to_bytes()
    }
}

impl FromBytes for BidStatus {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (result, bytes) = u8::from_bytes(bytes).unwrap();
        Ok((BidStatus::from(result), bytes))
    }
}

impl BidStatus {
    pub fn from(data: u8) -> Self {
        match data {
            0 => BidStatus::Pending,
            1 => BidStatus::Accepted,
            2 => BidStatus::NotAccepted,
            3 => BidStatus::Canceled,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Bid {
    pub offerer: AccountHash,
    pub price: U512,
    pub bid_time: u64,
    pub status: BidStatus,
}

impl CLTyped for Bid {
    fn cl_type() -> casper_types::CLType {
        CLType::Any
    }
}

impl FromBytes for Bid {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (offerer, bytes) = AccountHash::from_bytes(bytes)?;
        let (price, bytes) = U512::from_bytes(bytes)?;
        let (bid_time, bytes) = u64::from_bytes(bytes)?;
        let (status, bytes) = BidStatus::from_bytes(bytes)?;
        let body = Bid {
            offerer,
            price,
            bid_time,
            status,
        };
        Ok((body, bytes))
    }
}

impl ToBytes for Bid {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend(self.offerer.to_bytes()?);
        buffer.extend(self.price.to_bytes()?);
        buffer.extend(self.bid_time.to_bytes()?);
        buffer.extend(self.status.to_bytes()?);

        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.offerer.serialized_length()
            + self.price.serialized_length()
            + self.bid_time.serialized_length()
            + self.status.serialized_length()
    }
}
