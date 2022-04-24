using System;
using Avalonia.LogicalTree;
using Avalonia.Styling;
using FluentAvalonia.UI.Controls;

namespace Dimension.MaskCore.Lifecycle.Controls;

public class Dialog<T> : Dialog where T : ViewModel.ViewModel
{
    public T ViewModel => (DataContext as T)!;
}

public class Dialog : ContentDialog, IStyleable
{
    Type IStyleable.StyleKey => typeof(ContentDialog);

    protected override void OnDetachedFromLogicalTree(LogicalTreeAttachmentEventArgs e)
    {
        base.OnDetachedFromLogicalTree(e);
        if (DataContext is IDisposable disposable)
        {
            disposable.Dispose();
        }
    }
}