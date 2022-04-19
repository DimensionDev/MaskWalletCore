using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace Dimension.MaskWalletCore;

[TestClass]
public class PersonaKeyTest
{
    [TestMethod]
    public void TestCreate()
    {
        const string password = "123456";
        var mnemonic = WalletKey.GenerateMnemonic();
        const string path = "m/44'/60'/0'/0/0";
        const CurveType curveType = CurveType.Secp256k1;
        var option = new EncryptionOption(EncryptionOption.EncVersion.V38);
        var key = PersonaKey.Create(mnemonic, password, path, curveType, option);
        Assert.IsTrue(!string.IsNullOrEmpty(key.Identifier));
    }
}