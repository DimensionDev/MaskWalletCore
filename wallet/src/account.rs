use chain_common::coin::Coin;

pub struct Account {
    pub address: String,
    pub coin: Coin,
    pub derivationPath: String,
}