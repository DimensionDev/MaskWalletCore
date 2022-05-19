using Api;

namespace Dimension.MaskWalletCore;

public enum CurveType
{
    Secp256k1,
    Ed25519
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