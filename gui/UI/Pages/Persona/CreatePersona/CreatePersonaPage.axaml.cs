using Avalonia;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using Dimension.MaskCore.Lifecycle.Controls;

namespace Dimension.MaskCore.UI.Pages.Persona.CreatePersona;

internal partial class CreatePersonaPage : Page<CreatePersonaViewModel>
{
    public CreatePersonaPage()
    {
        InitializeComponent();
    }

    private void BackClicked(object? sender, RoutedEventArgs e)
    {
        GoBack();
    }
}