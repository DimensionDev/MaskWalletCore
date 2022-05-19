using Ethereum;

namespace Dimension.MaskWalletCore;

public interface IWalletSignInput
{
    SignInput GetSignInput();
}

public record EthereumSignInput(
    string Amount,
    ulong ChainId,
    string GasLimit,
    string GasPrice,
    string Nonce,
    byte[] Payload,
    string ToAddress,
    string MaxInclusionFeePerGas,
    string MaxFeePerGas
) : IWalletSignInput
{
    public SignInput GetSignInput()
    {
        return new SignInput
        {
            Amount = Amount,
            ChainId = ChainId,
            GasLimit = GasLimit,
            GasPrice = GasPrice,
            Nonce = Nonce,
            Payload = Payload,
            ToAddress = ToAddress,
            MaxInclusionFeePerGas = MaxInclusionFeePerGas,
            MaxFeePerGas = MaxFeePerGas
        };
    }
}