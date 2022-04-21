using System;
using System.Collections.Generic;
using System.Linq;
using System.Reactive.Linq;
using Realms;

namespace Dimension.MaskCore.Common.Extension;

public static class RxRealmExtension
{
    public static IObservable<IEnumerable<T>> AsObservable<T>(this IQueryable<T> query) where T : RealmObjectBase
    {
        return Observable.Create<IEnumerable<T>>(emitter => query.SubscribeForNotifications((sender, _, error) =>
        {
            if (error != null)
            {
                emitter.OnError(error);
            }
            else
            {
                emitter.OnNext(sender);
            }
        }));
    }
}