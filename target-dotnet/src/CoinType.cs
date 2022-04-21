using Api;

namespace Dimension.MaskWalletCore;

public enum CoinType
{
    Ethereum,
    Polkadot,
    Solana
}

internal static class CoinTypeExt
{
    public static Coin ToCoin(this CoinType coinType)
    {
        return coinType switch
        {
            CoinType.Ethereum => Coin.Ethereum,
            CoinType.Polkadot => Coin.Polkadot,
            CoinType.Solana => Coin.Solana,
            _ => throw new ArgumentOutOfRangeException(nameof(coinType), coinType, null)
        };
    }
}