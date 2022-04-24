using System;
using System.IO;
using System.Runtime.InteropServices;

namespace Dimension.MaskCore.Common;

internal class Consts
{
    public static string ConfigDirectory
    {
        get
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                return Path.Combine(
                    Environment.GetFolderPath(Environment.SpecialFolder.MyDocuments),
                    "Mask");
            }

            return Path.Combine(
                Environment.GetFolderPath(Environment.SpecialFolder.UserProfile),
                ".mask");
        }
    }
}