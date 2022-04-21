using Api;

namespace Dimension.MaskWalletCore;

public class PersonaKey
{
    private readonly PersonaGenerationResp _resp;

    private PersonaKey(PersonaGenerationResp resp)
    {
        _resp = resp;
    }

    public string Identifier => _resp.Identifier;

    public JsonWebKey PrivateKey => JsonWebKey.FromJWKResp(_resp.privateKey);

    public JsonWebKey PublicKey => JsonWebKey.FromJWKResp(_resp.publicKey);

    public JsonWebKey? LocalKey => _resp.localKey != null ? JsonWebKey.FromAesJWKResp(_resp.localKey) : null;

    public static PersonaKey Create(string mnemonic, string password, string path, CurveType curveType,
        EncryptionOption option)
    {
        var resp = Native.Call(new MWRequest
        {
            ParamGeneratePersona = new PersonaGenerationParam
            {
                Mnemonic = mnemonic,
                Password = password,
                Path = path,
                curve = curveType.ToCurve(),
                Option = option.ToEncryptOption()
            }
        }).RespGeneratePersona;
        return new PersonaKey(resp);
    }
}