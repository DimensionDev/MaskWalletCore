using System.Runtime.InteropServices;

namespace Dimension.MaskWalletCore;

[StructLayout(LayoutKind.Sequential)]
internal struct RustByteSlice : IDisposable
{
    public IntPtr bytes;
    public ulong len;

    public void Dispose()
    {
        rust_free(this);
    }

    public byte[] AsByteArray()
    {
        var buffer = new byte[len];
        Marshal.Copy(bytes, buffer, 0, buffer.Length);
        return buffer;
    }

    private const string LibName = "maskwalletcore_dotnet";

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void rust_free(RustByteSlice data);
}