using Api;

namespace Dimension.MaskWalletCore;

public interface IWalletValidate
{
    ValidateParam GetValidateParam();
}

public record PrivateKeyValidation(string PrivateKey) : IWalletValidate
{
    public ValidateParam GetValidateParam()
    {
        return new ValidateParam
        {
            privateKey = PrivateKey
        };
    }
}

public record MnemonicValidation(string Mnemonic) : IWalletValidate
{
    public ValidateParam GetValidateParam()
    {
        return new ValidateParam
        {
            Mnemonic = Mnemonic
        };
    }
}

public record KeyStoreJsonValidation(string KeyStoreJson) : IWalletValidate
{
    public ValidateParam GetValidateParam()
    {
        return new ValidateParam
        {
            keyStoreJSON = KeyStoreJson
        };
    }
}

public record StoredKeyValidation(byte[] StoredKey, string Password) : IWalletValidate
{
    public ValidateParam GetValidateParam()
    {
        return new ValidateParam
        {
            storedKeyPassword = new PasswordValidationParam
            {
                Password = Password,
                storedKeyData = StoredKey
            }
        };
    }
}

public record AddressValidation(string Address, CoinType Coin) : IWalletValidate
{
    public ValidateParam GetValidateParam()
    {
        return new ValidateParam
        {
            addressValidationParam = new AddressValidationParam
            {
                Address = Address,
                Coin = Coin.ToCoin()
            }
        };
    }
}