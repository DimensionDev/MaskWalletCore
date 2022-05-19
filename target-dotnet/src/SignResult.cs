namespace Dimension.MaskWalletCore;

public record SignResult(byte[] Encoded, uint V, byte[] R, byte[] S, byte[] Data);