using Avalonia;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using Dimension.MaskCore.Lifecycle.Controls;
using Dimension.MaskCore.UI.Pages.Persona.CreatePersona;

namespace Dimension.MaskCore.UI.Pages.Persona;

internal partial class PersonaPage : Page<PersonaViewModel>
{
    public PersonaPage()
    {
        InitializeComponent();
    }

    private void New_OnClicked(object? sender, RoutedEventArgs e)
    {
        Navigate<CreatePersonaPage>();
    }
}