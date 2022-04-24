using System;
using CommunityToolkit.Mvvm.ComponentModel;

namespace Dimension.MaskCore.Lifecycle.ViewModel;

[ObservableObject]
public abstract partial class ViewModel : IDisposable
{
    protected internal ViewModelScope Scope { get; } = new();

    public void Dispose()
    {
        Scope.Dispose();
    }
}

public interface IParameterizedViewModel<T>
{
    void Initialize(T parameter);
}

public abstract class ParameterizedViewModel<T> : ViewModel, IParameterizedViewModel<T>
{
    public void Initialize(T parameter)
    {
        InitializeCore(parameter);
    }

    protected abstract void InitializeCore(T parameter);
}