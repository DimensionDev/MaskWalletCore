using System.Collections.Immutable;
using Api;

namespace Dimension.MaskWalletCore;

public record CreateKeyResult(WalletKey Key, string mnemonic);

public class WalletKey
{
    private StoredKeyInfo _keyInfo;

    private WalletKey(StoredKeyInfo keyInfo)
    {
        _keyInfo = keyInfo;
    }

    public string Id => _keyInfo.Id;
    public string Hash => _keyInfo.Hash;
    public WalletKeyType Type => _keyInfo.Type.From();
    public byte[] Data => _keyInfo.Data;

    public static string GenerateMnemonic()
    {
        return Native.Call(new MWRequest
        {
            ParamGenerateMnemonic = new GenerateMnemonicParam()
        }).RespGenerateMnemonic.Mnemonic;
    }

    public static WalletKey? Load(byte[] key)
    {
        return Native.Call(new MWRequest
        {
            ParamLoadStoredKey = new LoadStoredKeyParam
            {
                Datas = { key }
            }
        }).RespLoadStoredKey.StoredKeys.Select(it => new WalletKey(it)).FirstOrDefault();
    }

    public static CreateKeyResult Create(string password)
    {
        var result = Native.Call(new MWRequest
        {
            ParamCreateStoredKey = new CreateStoredKeyParam
            {
                Password = password
            }
        }).RespCreateStoredKey;
        return new CreateKeyResult(new WalletKey(result.StoredKey), result.Mnemonic);
    }

    public static WalletKey FromMnemonic(string mnemonic, string password)
    {
        return new WalletKey(Native.Call(new MWRequest
        {
            ParamImportMnemonic = new ImportMnemonicStoredKeyParam
            {
                Mnemonic = mnemonic,
                Password = password
            }
        }).RespImportMnemonic.StoredKey);
    }

    public static WalletKey FromJson(string json, string password, string name, string keyStoreJsonPassword,
        CoinType coin)
    {
        return new WalletKey(Native.Call(new MWRequest
        {
            ParamImportJson = new ImportJSONStoredKeyParam
            {
                Json = json,
                Password = password,
                Name = name,
                keyStoreJsonPassword = keyStoreJsonPassword,
                Coin = coin.ToCoin()
            }
        }).RespImportJson.StoredKey);
    }

    public static WalletKey FromPrivateKey(string privateKey, string password, string name, CoinType coin)
    {
        return new WalletKey(Native.Call(new MWRequest
        {
            ParamImportPrivateKey = new ImportPrivateStoredKeyParam
            {
                privateKey = privateKey,
                Password = password,
                Name = name,
                Coin = coin.ToCoin()
            }
        }).RespImportPrivateKey.StoredKey);
    }

    public static bool Validate(IWalletValidate walletValidate)
    {
        return Native.Call(new MWRequest
        {
            ParamValidation = walletValidate.GetValidateParam()
        }).RespValidate.Valid;
    }

    public static IReadOnlyCollection<ImportExportType> SupportedImportType(CoinType coinType)
    {
        return Native.Call(new MWRequest
        {
            ParamGetStoredKeyImportType = new GetKeyStoreSupportImportTypeParam
            {
                Coin = coinType.ToCoin()
            }
        }).RespGetStoredKeyImportType.Types.Select(it => it.FromImportType()).ToImmutableList();
    }

    public static IReadOnlyCollection<ImportExportType> SupportedExportType(CoinType coinType)
    {
        return Native.Call(new MWRequest
        {
            ParamGetStoredKeyExportType = new GetKeyStoreSupportExportTypeParam
            {
                Coin = coinType.ToCoin()
            }
        }).RespGetStoredKeyExportType.Types.Select(it => it.FromImportType()).ToImmutableList();
    }

    public WalletAccount AddNewAccountAtPath(CoinType coinType, string derivationPath, string name, string password)
    {
        var result = Native.Call(new MWRequest
        {
            ParamCreateAccountOfCoinAtPath = new CreateStoredKeyNewAccountAtPathParam
            {
                Name = name,
                Password = password,
                Coin = coinType.ToCoin(),
                derivationPath = derivationPath,
                StoredKeyData = _keyInfo.Data
            }
        }).RespCreateAccountOfCoinAtPath;
        _keyInfo = result.storedKey;
        return new WalletAccount(result.Account);
    }

    public string ExportMnemonic(string password)
    {
        return Native.Call(new MWRequest
        {
            ParamExportMnemonic = new ExportKeyStoreMnemonicParam
            {
                Password = password,
                StoredKeyData = _keyInfo.Data
            }
        }).RespExportMnemonic.Mnemonic;
    }

    public string ExportPrivateKey(string password, CoinType coinType)
    {
        return Native.Call(new MWRequest
        {
            ParamExportPrivateKey = new ExportKeyStorePrivateKeyParam
            {
                Password = password,
                StoredKeyData = _keyInfo.Data,
                Coin = coinType.ToCoin()
            }
        }).RespExportPrivateKey.privateKey;
    }

    public string ExportPrivateKeyAtPath(string password, string derivationPath, CoinType coinType)
    {
        return Native.Call(new MWRequest
        {
            ParamExportPrivateKeyOfPath = new ExportKeyStorePrivateKeyOfPathParam
            {
                Password = password,
                StoredKeyData = _keyInfo.Data,
                Coin = coinType.ToCoin(),
                derivationPath = derivationPath
            }
        }).RespExportPrivateKey.privateKey;
    }

    public string ExportKeyStoreJsonOfAddress(string address, CoinType coinType, string password, string newPassword)
    {
        return Native.Call(new MWRequest
        {
            ParamExportKeyStoreJsonOfAddress = new ExportKeyStoreJSONOfAddressParam
            {
                Password = password,
                StoredKeyData = _keyInfo.Data,
                Coin = coinType.ToCoin(),
                Address = address,
                newPassword = newPassword
            }
        }).RespExportKeyStoreJson.Json;
    }

    public string ExportKeyStoreJsonOfPath(string derivationPath, CoinType coinType, string password,
        string newPassword)
    {
        return Native.Call(new MWRequest
        {
            ParamExportKeyStoreJsonOfPath = new ExportKeyStoreJSONOfPathParam
            {
                Password = password,
                StoredKeyData = _keyInfo.Data,
                Coin = coinType.ToCoin(),
                derivationPath = derivationPath,
                newPassword = newPassword
            }
        }).RespExportKeyStoreJson.Json;
    }

    public void UpdatePassword(string oldPassword, string newPassword)
    {
        var result = Native.Call(new MWRequest
        {
            ParamUpdateKeyStorePassword = new UpdateStoredKeyPasswordParam
            {
                StoredKeyData = _keyInfo.Data,
                newPassword = newPassword,
                oldPassword = oldPassword
            }
        }).RespUpdateKeyStorePassword;
        _keyInfo = result.StoredKey;
    }

    public SignResult Sign(string derivationPath, string password, CoinType coinType, IWalletSignInput input)
    {
        var result = Native.Call(new MWRequest
        {
            ParamSignTransaction = new SignTransactionParam
            {
                storedKeyData = _keyInfo.Data,
                Coin = coinType.ToCoin(),
                derivationPath = derivationPath,
                Password = password,
                SignInput = input.GetSignInput()
            }
        }).RespSignTransaction.SignOutput;
        return new SignResult(result.Encoded, result.V, result.R, result.S, result.Data);
    }
}