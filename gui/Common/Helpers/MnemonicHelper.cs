using Dimension.MaskWalletCore;

namespace Dimension.MaskCore.Common.Helpers;

internal static class MnemonicHelper
{
    public static string GenerateMnemonic() => WalletKey.GenerateMnemonic();
}