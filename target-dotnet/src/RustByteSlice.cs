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
    
    private const string LibName = "maskwalletcore_dotnet";
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void rust_free(RustByteSlice data);
}
