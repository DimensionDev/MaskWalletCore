using System.Runtime.Serialization;

namespace Dimension.MaskWalletCore;

[Serializable]
public class MaskWalletCoreException : Exception
{
    //
    // For guidelines regarding the creation of new exception types, see
    //    http://msdn.microsoft.com/library/default.asp?url=/library/en-us/cpgenref/html/cpconerrorraisinghandlingguidelines.asp
    // and
    //    http://msdn.microsoft.com/library/default.asp?url=/library/en-us/dncscol/html/csharp07192001.asp
    //

    public MaskWalletCoreException()
    {
    }

    public MaskWalletCoreException(string message) : base(message)
    {
    }

    public MaskWalletCoreException(string message, Exception inner) : base(message, inner)
    {
    }

    protected MaskWalletCoreException(
        SerializationInfo info,
        StreamingContext context) : base(info, context)
    {
    }
}