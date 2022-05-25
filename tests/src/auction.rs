use alloc::vec::Vec;
use casper_types::{
    account::AccountHash,
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped, ContractHash, U256, U512,
};

use crate::bid::Bid;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum AuctionType {
    Basic = 0,
    FixedPrice = 1,
    NoReserve = 2,
    Decreasing = 3,
}

impl CLTyped for AuctionType {
    fn cl_type() -> CLType {
        CLType::U8
    }
}

impl ToBytes for AuctionType {
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

impl FromBytes for AuctionType {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (result, bytes) = u8::from_bytes(bytes).unwrap();
        Ok((AuctionType::from(result), bytes))
    }
}

impl AuctionType {
    pub fn from(data: u8) -> Self {
        match data {
            0 => AuctionType::Basic,
            1 => AuctionType::FixedPrice,
            2 => AuctionType::NoReserve,
            3 => AuctionType::Decreasing,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Auction {
    pub maker: AccountHash,
    pub collection: ContractHash,
    pub token_id: U256,
    pub auction_type: AuctionType,
    pub price: Option<U512>,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub bids: Vec<Bid>,
}

impl CLTyped for Auction {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

impl ToBytes for Auction {
    fn to_bytes(&self) -> Result<alloc::vec::Vec<u8>, casper_types::bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend(self.maker.to_bytes()?);
        buffer.extend(self.collection.to_bytes()?);
        buffer.extend(self.token_id.to_bytes()?);
        buffer.extend(self.auction_type.to_bytes()?);
        buffer.extend(self.price.to_bytes()?);
        buffer.extend(self.start_time.to_bytes()?);
        buffer.extend(self.end_time.to_bytes()?);
        buffer.extend(self.bids.to_bytes()?);
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.maker.serialized_length()
            + self.collection.serialized_length()
            + self.token_id.serialized_length()
            + self.auction_type.serialized_length()
            + self.price.serialized_length()
            + self.start_time.serialized_length()
            + self.end_time.serialized_length()
            + self.bids.serialized_length()
    }

    fn into_bytes(self) -> Result<alloc::vec::Vec<u8>, bytesrepr::Error>
    where
        Self: Sized,
    {
        self.to_bytes()
    }
}

impl FromBytes for Auction {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (maker, bytes) = AccountHash::from_bytes(bytes)?;
        let (collection, bytes) = ContractHash::from_bytes(bytes)?;
        let (token_id, bytes) = U256::from_bytes(bytes)?;
        let (auction_type, bytes) = AuctionType::from_bytes(bytes)?;
        let (price, bytes) = Option::<U512>::from_bytes(bytes)?;
        let (start_time, bytes) = u64::from_bytes(bytes)?;
        let (end_time, bytes) = Option::<u64>::from_bytes(bytes)?;
        let (bids, bytes) = Vec::<Bid>::from_bytes(bytes)?;
        Ok((
            Auction {
                maker,
                collection,
                token_id,
                auction_type,
                price,
                start_time,
                end_time,
                bids,
            },
            bytes,
        ))
    }
}
