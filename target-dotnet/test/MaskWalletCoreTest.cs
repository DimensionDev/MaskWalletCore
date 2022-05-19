using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace Dimension.MaskWalletCore;

[TestClass]
public class MaskWalletCoreTest
{
    [TestMethod]
    public void TestWalletKeyCreation()
    {
        const string password = "password";
        var result = WalletKey.Create(password);
        Assert.IsFalse(string.IsNullOrEmpty(result.mnemonic));
        Assert.IsFalse(string.IsNullOrWhiteSpace(result.mnemonic));
        Assert.IsFalse(string.IsNullOrEmpty(result.Key.Id));
        Assert.IsFalse(string.IsNullOrEmpty(result.Key.Hash));
    }

    [TestMethod]
    public void TestWalletRestoreByMnemonic()
    {
        const string password = "password";
        var result = WalletKey.Create(password);
        var wallet = WalletKey.FromMnemonic(result.mnemonic, password);
        Assert.IsNotNull(wallet);
        Assert.IsFalse(string.IsNullOrEmpty(wallet.Id));
        Assert.IsFalse(string.IsNullOrEmpty(wallet.Hash));
    }

    [TestMethod]
    public void TestPersonaCreate()
    {
        const string password = "123456";
        var mnemonic = WalletKey.GenerateMnemonic();
        const string path = "m/44'/60'/0'/0/0";
        const CurveType curveType = CurveType.Secp256k1;
        var option = new EncryptionOption(EncryptionOption.EncVersion.V38);
        var key = PersonaKey.Create(mnemonic, password, path, curveType, option);
        Assert.IsFalse(string.IsNullOrEmpty(key.Identifier));
        Assert.IsTrue(key.Identifier.StartsWith("ec_key:secp256k1/"));
    }
}