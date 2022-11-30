using System.Runtime.InteropServices;
using Api;
using ProtoBuf;

namespace Dimension.MaskWalletCore;

internal class Native
{
    private const string LibName = "maskwalletcore_dotnet";

    public static MWResponse Call(MWRequest request)
    {
        using var stream = new MemoryStream();
        Serializer.Serialize(stream, request);
        var data = stream.ToArray();
        using var slice = rust_request(data, Convert.ToUInt64(data.Length));
        var buffer = slice.AsByteArray();
        using var responseStream = new MemoryStream(buffer);
        var result = Serializer.Deserialize<MWResponse>(responseStream);
        if (result.Error != null)
        {
            throw new MaskWalletCoreException(result.Error.errorMsg);
        }

        return result;
    }

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    private static extern RustByteSlice rust_request([In] byte[] data, ulong size);
}