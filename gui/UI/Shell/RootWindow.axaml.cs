using Avalonia;
using FluentAvalonia.UI.Controls;

namespace Dimension.MaskCore.UI.Shell;

internal partial class RootWindow : CoreWindow
{
    public RootWindow()
    {
        InitializeComponent();
#if DEBUG
        this.AttachDevTools();
#endif
    }
}