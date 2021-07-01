use chain_common::ethereum::SignInput;
use chain_common::Error;
use ethereum_types::{H160, U256};
use std::convert::TryFrom;
use std::str::FromStr;

pub struct Transaction {
    pub chain_id: u64,
    /// The address of the receiver
    pub receiver: Option<H160>,
    /// The amount (in wei)
    pub amount: U256,
    /// The transaction gas limit
    pub gas_limit: U256,
    /// The transaction gas price in wei
    pub gas_price: U256,
    /// The nonce of the Ethereum account
    pub nonce: U256,
    /// The transaction data
    pub payload: Vec<u8>,
}

impl TryFrom<&SignInput> for Transaction {
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

        Ok(Transaction {
            chain_id: input.chain_id,
            receiver,
            amount,
            gas_limit,
            gas_price,
            nonce,
            payload: input.payload.to_vec(),
        })
    }
}
