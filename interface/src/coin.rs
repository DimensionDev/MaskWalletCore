use chain_common::coin::Coin;

use super::param::Coin as ProtoCoin;

pub fn prost_coin_to_chain_coin(prost_coin: ProtoCoin) -> Coin {
    Coin {
        id: prost_coin.id,
        name: prost_coin.name,
        coin_id: prost_coin.coin_id,
        symbol: prost_coin.symbol,
        decimal: prost_coin.decimal,
        blockchain: prost_coin.blockchain,
        derivation_path: prost_coin.derivation_path,
        curve: prost_coin.curve,
        public_key_type: prost_coin.public_key_type
    }
}

// impl ProtoCoin {
//     pub fn to_chain_coin(&mut self) -> Coin {
//         Coin {
//             id: self.id,
//             name: self.name,
//             coin_id: self.coin_id,
//             symbol: self.symbol,
//             decimal: self.decimal,
//             blockchain: self.blockchain,
//             derivation_path: self.derivation_path,
//             curve: self.curve,
//             public_key_type: self.public_key_type
//         }
//     }
// }