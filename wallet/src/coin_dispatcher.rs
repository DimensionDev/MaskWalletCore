use chain_common::entry::Entry;
use chain_common::coin::Coin;

use ethereum::entry::EthereumEntry;

pub fn get_dispatcher(coin: &Coin) -> Box<dyn Entry> {
    match coin.name.as_str() {
        "ethereum" => Box::new(EthereumEntry{}),
        _ => Box::new(EthereumEntry{})
    }
}