using Api;

namespace Dimension.MaskWalletCore;

public enum WalletKeyType
{
    PrivateKey,
    Mnemonic
}

internal static class WalletKeyTypeExt
{
    public static WalletKeyType From(this StoredKeyType type)
    {
        return type switch
        {
            StoredKeyType.PrivateKey => WalletKeyType.PrivateKey,
            StoredKeyType.Mnemonic => WalletKeyType.Mnemonic,
            _ => throw new ArgumentOutOfRangeException(nameof(type), type, null)
        };
    }
}