using System;
using Avalonia;
using Avalonia.Controls;
using Avalonia.LogicalTree;
using Avalonia.VisualTree;
using FluentAvalonia.UI.Controls;
using FluentAvalonia.UI.Media.Animation;
using FluentAvalonia.UI.Navigation;

namespace Dimension.MaskCore.Lifecycle.Controls;

public class Page<T> : Page where T : ViewModel.ViewModel, new()
{
    public T ViewModel => (DataContext as T)!;
}

public class Page : UserControl
{
    protected Frame? Frame { get; private set; }

    protected Window Window => this.FindAncestorOfType<Window>();

    protected override void OnAttachedToVisualTree(VisualTreeAttachmentEventArgs e)
    {
        base.OnAttachedToVisualTree(e);
        Frame = this.FindAncestorOfType<Frame>();
        Frame.Navigated += OnNavigated;
    }

    private void OnNavigated(object sender, NavigationEventArgs e)
    {
        if (ReferenceEquals(e.Content, this))
        {
            switch (e.NavigationMode)
            {
                case NavigationMode.New:
                    OnCreated(e.Parameter);
                    break;
                case NavigationMode.Back:
                    break;
                case NavigationMode.Forward:
                    break;
                case NavigationMode.Refresh:
                    break;
                default:
                    throw new ArgumentOutOfRangeException();
            }
        }
    }
    
    protected virtual void OnCreated(object parameter)
    {
    }
    
    protected override void OnDetachedFromLogicalTree(LogicalTreeAttachmentEventArgs e)
    {
        base.OnDetachedFromLogicalTree(e);
        if (DataContext is IDisposable disposable)
        {
            disposable.Dispose();
        }

        if (Frame != null)
        {
            Frame.Navigated -= OnNavigated;
            Frame = null;
        }
    }

    protected void GoBack()
    {
        this.FindAncestorOfType<Frame>().GoBack(new SlideNavigationTransitionInfo
            { Effect = SlideNavigationTransitionEffect.FromLeft });
    }

    protected void Navigate<T>(object? parameter = null)
    {
        Frame?.Navigate(typeof(T), parameter, new SlideNavigationTransitionInfo());
    }
}