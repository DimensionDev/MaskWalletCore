using Api;

namespace Dimension.MaskWalletCore;

public record EncryptionOption(EncryptionOption.EncVersion Version)
{
    public enum EncVersion
    {
        V37,
        V38
    }

    public EncryptOption ToEncryptOption()
    {
        return new EncryptOption
        {
            version = Version.ToVersion()
        };
    }
}

internal static class EncVersionExt
{
    public static EncryptOption.Version ToVersion(this EncryptionOption.EncVersion version)
    {
        return version switch
        {
            EncryptionOption.EncVersion.V37 => EncryptOption.Version.V37,
            EncryptionOption.EncVersion.V38 => EncryptOption.Version.V38,
            _ => throw new ArgumentOutOfRangeException(nameof(version), version, null)
        };
    }
}