using System;
using System.Collections.Generic;
using System.Linq;

namespace Dimension.MaskCore.Common.Extension;

internal static class EnumerabsleExtension
{
    public static IEnumerable<T> Randomize<T>(this IEnumerable<T> source)
    {
        var rnd = new Random();
        return source.OrderBy(item => rnd.Next());
    }
}