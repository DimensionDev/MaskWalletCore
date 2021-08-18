use super::eip2930::AccessList;
use chain_common::ethereum::SignInput;
use chain_common::Error;
use crypto::hash::{Hasher, Keccak256};
use crypto::Error as CryptoError;
use ethereum_types::{H160, U256};
use rlp::RlpStream;
use std::convert::TryFrom;
use std::str::FromStr;

pub trait Transaction {
    fn uses_replay_protection(&self) -> bool;
    fn hash(&self, chain_id: u64) -> Result<Vec<u8>, CryptoError>;
    fn encode_transaction(&self, v: u64, r: &[u8], s: &[u8], chain_id: u64) -> Vec<u8>;
}

pub struct TransactionBase {
    pub nonce: U256,
    pub payload: Vec<u8>,

    /// The transaction gas limit
    pub gas_limit: U256,

    /// The address of the receiver
    pub receiver: Option<H160>,

    /// The amount (in wei)
    pub amount: U256,
}

pub struct LegacyTransaction {
    pub base: TransactionBase,

    /// The transaction gas price in wei
    pub gas_price: U256,
}

pub struct Eip1559Transaction {
    pub base: TransactionBase,

    pub max_inclusion_fee_per_gas: U256,

    pub max_fee_per_gas: U256,
}

impl TryFrom<&SignInput> for LegacyTransaction {
    type Error = Error;
    fn try_from(input: &SignInput) -> Result<Self, Self::Error> {
        // Trim the '0x' prefix
        let trimmed_address = match input.to_address.starts_with("0x") {
            true => input.to_address[2..].to_string(),
            false => input.to_address.to_string(),
        };
        let amount =
            U256::from_str_radix(&input.amount, 16).map_err(|_| Error::InvalidSignInput)?;
        let gas_limit =
            U256::from_str_radix(&input.gas_limit, 16).map_err(|_| Error::InvalidSignInput)?;
        let gas_price =
            U256::from_str_radix(&input.gas_price, 16).map_err(|_| Error::InvalidSignInput)?;
        let nonce = U256::from_str_radix(&input.nonce, 16).map_err(|_| Error::InvalidSignInput)?;
        let receiver: Option<H160> = match trimmed_address.is_empty() {
            true => None,
            false => Some(H160::from_str(&trimmed_address).map_err(|_| Error::InvalidSignInput)?),
        };

        let base = TransactionBase {
            nonce,
            payload: input.payload.to_vec(),
            gas_limit,
            receiver,
            amount,
        };

        Ok(LegacyTransaction { base, gas_price })
    }
}

impl TryFrom<&SignInput> for Eip1559Transaction {
    type Error = Error;
    fn try_from(input: &SignInput) -> Result<Self, Self::Error> {
        // Trim the '0x' prefix
        let trimmed_address = match input.to_address.starts_with("0x") {
            true => input.to_address[2..].to_string(),
            false => input.to_address.to_string(),
        };
        let amount =
            U256::from_str_radix(&input.amount, 16).map_err(|_| Error::InvalidSignInput)?;
        let gas_limit =
            U256::from_str_radix(&input.gas_limit, 16).map_err(|_| Error::InvalidSignInput)?;
        let max_fee_per_gas = U256::from_str_radix(&input.max_fee_per_gas, 16)
            .map_err(|_| Error::InvalidSignInput)?;
        let max_inclusion_fee_per_gas = U256::from_str_radix(&input.max_inclusion_fee_per_gas, 16)
            .map_err(|_| Error::InvalidSignInput)?;
        let nonce = U256::from_str_radix(&input.nonce, 16).map_err(|_| Error::InvalidSignInput)?;
        let receiver: Option<H160> = match trimmed_address.is_empty() {
            true => None,
            false => Some(H160::from_str(&trimmed_address).map_err(|_| Error::InvalidSignInput)?),
        };

        let base = TransactionBase {
            nonce,
            payload: input.payload.to_vec(),
            gas_limit,
            receiver,
            amount,
        };

        Ok(Eip1559Transaction {
            base,
            max_inclusion_fee_per_gas,
            max_fee_per_gas,
        })
    }
}

impl Transaction for LegacyTransaction {
    fn uses_replay_protection(&self) -> bool {
        true
    }

    fn hash(&self, chain_id: u64) -> Result<Vec<u8>, CryptoError> {
        let mut rlp_stream = RlpStream::new_list(9);
        rlp_stream.append(&self.base.nonce);
        rlp_stream.append(&self.gas_price);
        rlp_stream.append(&self.base.gas_limit);
        if let Some(ref t) = self.base.receiver {
            rlp_stream.append(t);
        } else {
            rlp_stream.append(&vec![]);
        }
        rlp_stream.append(&self.base.amount);
        rlp_stream.append(&self.base.payload);
        rlp_stream.append(&chain_id);
        rlp_stream.append(&U256::zero());
        rlp_stream.append(&U256::zero());
        let encoded = rlp_stream.out();
        Hasher::hash(Keccak256, &encoded)
    }

    fn encode_transaction(&self, v: u64, r: &[u8], s: &[u8], _chain_id: u64) -> Vec<u8> {
        let mut rlp_stream = RlpStream::new_list(9);
        rlp_stream.append(&self.base.nonce);
        rlp_stream.append(&self.gas_price);
        rlp_stream.append(&self.base.gas_limit);
        if let Some(ref t) = self.base.receiver {
            rlp_stream.append(t);
        } else {
            rlp_stream.append(&vec![]);
        }
        rlp_stream.append(&self.base.amount);
        rlp_stream.append(&self.base.payload);
        rlp_stream.append(&v);
        rlp_stream.append(&r);
        rlp_stream.append(&s);
        rlp_stream.out().to_vec()
    }
}

impl Transaction for Eip1559Transaction {
    fn uses_replay_protection(&self) -> bool {
        false
    }

    fn hash(&self, chain_id: u64) -> Result<Vec<u8>, CryptoError> {
        let mut rlp_stream = RlpStream::new_list(9);
        rlp_stream.append(&chain_id);
        rlp_stream.append(&self.base.nonce);
        rlp_stream.append(&self.max_inclusion_fee_per_gas);
        rlp_stream.append(&self.max_fee_per_gas);
        rlp_stream.append(&self.base.gas_limit);
        if let Some(ref t) = self.base.receiver {
            rlp_stream.append(t);
        } else {
            rlp_stream.append(&vec![]);
        }
        rlp_stream.append(&self.base.amount);
        rlp_stream.append(&self.base.payload);
        let access_list = AccessList::from(vec![]);
        rlp_stream.append(&access_list);
        let rlp_encoded = rlp_stream.out();

        let mut encoded = vec![];
        encoded.extend_from_slice(&[0x2]);
        encoded.extend_from_slice(&rlp_encoded);
        Hasher::hash(Keccak256, &encoded)
    }

    fn encode_transaction(&self, v: u64, r: &[u8], s: &[u8], chain_id: u64) -> Vec<u8> {
        let mut rlp_stream = RlpStream::new();
        rlp_stream.begin_unbounded_list();
        rlp_stream.append(&chain_id);
        rlp_stream.append(&self.base.nonce);
        rlp_stream.append(&self.max_inclusion_fee_per_gas);
        rlp_stream.append(&self.max_fee_per_gas);
        rlp_stream.append(&self.base.gas_limit);
        if let Some(ref t) = self.base.receiver {
            rlp_stream.append(t);
        } else {
            rlp_stream.append(&vec![]);
        }
        rlp_stream.append(&self.base.amount);
        rlp_stream.append(&self.base.payload);
        let access_list = AccessList::from(vec![]);
        rlp_stream.append(&access_list);
        rlp_stream.append(&v);
        rlp_stream.append(&r);
        rlp_stream.append(&s);
        rlp_stream.finalize_unbounded_list();

        let rlp_encoded = rlp_stream.out().freeze();
        let mut encoded = vec![];
        encoded.extend_from_slice(&[0x2]);
        encoded.extend_from_slice(&rlp_encoded);
        encoded.to_vec()
    }
}
