using ProtoBuf;

namespace Dimension.MaskWalletCore;

using System.Runtime.InteropServices;

internal class Native
{
    public static Api.MWResponse Call(Api.MWRequest request)
    {
        using var stream = new MemoryStream();
        Serializer.Serialize(stream, request);
        stream.Position = 0L;
        var data = stream.ToArray();
        using var slice = rust_request(data, Convert.ToUInt64(data.Length));
        var buffer = new byte[slice.len];
        Marshal.Copy(slice.bytes, buffer, 0, buffer.Length);
        using var responseStream = new MemoryStream(buffer);
        responseStream.Position = 0L;
        var result = Serializer.Deserialize<Api.MWResponse>(responseStream);
        if (result.Error != null)
        {
            throw new MaskWalletCoreException(result.Error.errorMsg);
        }
        return result;
    }

    private const string LibName = "maskwalletcore_dotnet";

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    private static extern RustByteSlice rust_request([In] byte[] data, ulong size);
}