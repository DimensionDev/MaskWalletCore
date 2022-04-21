using Api;

namespace Dimension.MaskWalletCore;

public class WalletAccount
{
    private readonly StoredKeyAccountInfo _account;

    internal WalletAccount(StoredKeyAccountInfo account)
    {
        _account = account;
    }

    public string Address => _account.Address;
    public string Name => _account.Name;
    public string Coin => _account.Coin;
    public string DerivationPath => _account.derivationPath;
    public string ExtendedPublicKey => _account.extendedPublicKey;
}