use alloc::vec::Vec;

use casper_types::{
    account::AccountHash,
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped, ContractHash, U256,
};

use super::bid::Bid;

#[derive(Clone, Debug)]
pub struct Offer {
    pub collection: ContractHash,
    pub token_id: U256,
    pub bids: Vec<Bid>,
}

impl CLTyped for Offer {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

impl FromBytes for Offer {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (collection, bytes) = ContractHash::from_bytes(bytes)?;
        let (token_id, bytes) = U256::from_bytes(bytes)?;
        let (bids, bytes) = Vec::<Bid>::from_bytes(bytes)?;

        let body = Offer {
            collection,
            token_id,
            bids,
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

        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.collection.serialized_length()
            + self.token_id.serialized_length()
            + self.bids.serialized_length()
    }
}

impl PartialEq for Offer {
    fn eq(&self, other: &Self) -> bool {
        let eq_collection = self.collection.eq(&other.collection);
        let eq_token_id = self.token_id.eq(&other.token_id);

        let matching = self
            .bids
            .iter()
            .zip(&other.bids)
            .filter(|&(a, b)| a == b)
            .count();
        let eq_bids = matching == self.bids.len();

        eq_collection && eq_token_id && eq_bids
    }
}

impl Offer {
    pub fn get_bid_index_by_account(&self, account: AccountHash) -> Option<usize> {
        let mut index: usize = 0;
        for bid in &self.bids {
            if bid.offerer == account {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}
