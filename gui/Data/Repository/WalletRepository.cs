using Dimension.MaskWalletCore;

namespace Dimension.MaskCore.Data.Repository;

internal class WalletRepository
{
    public string GenerateMnemonic()
    {
        return WalletKey.GenerateMnemonic();
    }
}