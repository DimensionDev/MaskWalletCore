using System;
using System.Collections.Generic;

namespace Dimension.MaskCore.Lifecycle.ViewModel;

public sealed class ViewModelScope : IDisposable
{
    private readonly List<IDisposable> _disposables = new();

    public void Dispose()
    {
        _disposables.ForEach(x => x.Dispose());
    }

    internal void Add(IDisposable disposable)
    {
        _disposables.Add(disposable);
    }
}

public static class RxExtensions
{
    public static void SubscribeIn<T>(this IObservable<T> source, ViewModel viewModel, Action<T> onNext)
    {
        viewModel.Scope.Add(source.Subscribe(onNext));
    }
}