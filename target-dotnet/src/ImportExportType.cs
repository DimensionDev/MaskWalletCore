using Api;

namespace Dimension.MaskWalletCore;

public enum ImportExportType
{
    PrivateKey,
    Mnemonic,
    KeyStoreJSON
}

internal static class ImportExportTypeExt
{
    public static StoredKeyImportType ToImportType(this ImportExportType type)
    {
        return type switch
        {
            ImportExportType.PrivateKey => StoredKeyImportType.PrivateKeyImportType,
            ImportExportType.Mnemonic => StoredKeyImportType.MnemonicImportType,
            ImportExportType.KeyStoreJSON => StoredKeyImportType.KeyStoreJSONImportType
        };
    }

    public static ImportExportType FromImportType(this StoredKeyImportType type)
    {
        return type switch
        {
            StoredKeyImportType.PrivateKeyImportType => ImportExportType.PrivateKey,
            StoredKeyImportType.MnemonicImportType => ImportExportType.Mnemonic,
            StoredKeyImportType.KeyStoreJSONImportType => ImportExportType.KeyStoreJSON,
            _ => throw new ArgumentOutOfRangeException(nameof(type), type, null)
        };
    }
}