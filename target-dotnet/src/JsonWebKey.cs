using Api;

namespace Dimension.MaskWalletCore;

public record JsonWebKey(
    string? kty = null,
    string? kid = null,
    string? use = null,
    List<string>? key_ops = null,
    string? alg = null,
    bool? ext = null,
    string? crv = null,
    string? x = null,
    string? y = null,
    string? d = null,
    string? n = null,
    string? e = null,
    string? p = null,
    string? q = null,
    string? dp = null,
    string? dq = null,
    string? qi = null,
    List<JsonWebKey.RsaOtherPrimesInfo>? oth = null,
    string? k = null
)
{
    public static JsonWebKey FromJWKResp(JWKResp resp)
    {
        return new JsonWebKey(
            resp.Kty,
            key_ops: resp.KeyOps,
            ext: resp.Ext,
            crv: resp.Crv,
            x: resp.X,
            y: resp.Y,
            d: resp.D
        );
    }

    public static JsonWebKey FromAesJWKResp(AesJWKResp resp)
    {
        return new JsonWebKey(
            resp.Kty,
            key_ops: resp.KeyOps,
            alg: resp.Alg,
            ext: resp.Ext,
            k: resp.K
        );
    }

    public record RsaOtherPrimesInfo(string r, string d, string t);
}