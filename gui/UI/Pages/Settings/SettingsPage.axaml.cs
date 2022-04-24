using System.Diagnostics;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Dimension.MaskCore.Lifecycle.Controls;

namespace Dimension.MaskCore.UI.Pages.Settings;

internal partial class SettingsPage : Page<SettingsViewModel>
{
    public SettingsPage()
    {
        InitializeComponent();
    }

    private void ContactButton_Clicked(object? sender, RoutedEventArgs e)
    {
        e.Handled = true;
        if (sender is Button { Tag: string url } && !string.IsNullOrEmpty(url))
        {
            Process.Start(new ProcessStartInfo
            {
                FileName = url,
                UseShellExecute = true
            })?.Dispose();
        }
    }
}