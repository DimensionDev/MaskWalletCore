using Api;

namespace Dimension.MaskWalletCore;

public class WalletKey
{
    public static string GenerateMnemonic()
    {
        return Native.Call(new MWRequest
        {
            ParamGenerateMnemonic = new GenerateMnemonicParam()
        }).RespGenerateMnemonic.Mnemonic;
    }
}

public record JsonWebKey(
    string? kty,
    string? kid,
    string? use,
    List<String>? key_ops,
    string? alg,
    Boolean? ext,
    string? crv,
    string? x,
    string? y,
    string? d,
    string? n,
    string? e,
    string? p,
    string? q,
    string? dp,
    string? dq,
    string? qi,
    List<JsonWebKey.RsaOtherPrimesInfo>? oth,
    string? k
)
{
    public record RsaOtherPrimesInfo(string r, string d, string t);
}