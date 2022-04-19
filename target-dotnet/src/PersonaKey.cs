using Api;

namespace Dimension.MaskWalletCore;

public enum CurveType
{
    Secp256k1,
    Ed25519,
}

internal static class CurveTypeExt 
{
    public static PersonaGenerationParam.Curve ToCurve(this CurveType type)
    {
        return type switch
        {
            CurveType.Secp256k1 => PersonaGenerationParam.Curve.Secp256k1,
            CurveType.Ed25519 => PersonaGenerationParam.Curve.Ed25519,
            _ => throw new ArgumentOutOfRangeException(nameof(type), type, null)
        };
    }
}

public record EncryptionOption(EncryptionOption.EncVersion Version)
{
    public enum EncVersion
    {
        V37,
        V38,
    }
    public EncryptOption ToEncryptOption()
    {
        return new EncryptOption
        {
            version = this.Version.ToVersion(),
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

public class PersonaKey
{
    private readonly PersonaGenerationResp _resp;

    private PersonaKey(PersonaGenerationResp resp)
    {
        _resp = resp;
    }

    public static PersonaKey Create(string mnemonic, string password, string path, CurveType curveType, EncryptionOption option)
    {
        var resp = Native.Call(new MWRequest
        {
            ParamGeneratePersona = new PersonaGenerationParam
            {
                Mnemonic = mnemonic,
                Password = password,
                Path = path,
                curve = curveType.ToCurve(),
                Option = option.ToEncryptOption(),
            }
        }).RespGeneratePersona;
        return new PersonaKey(resp);
    }
    
    public string Identifier => _resp.Identifier;
}