using Avalonia.Interactivity;
using Dimension.MaskCore.Lifecycle.Controls;
using Dimension.MaskCore.UI.Model;
using Dimension.MaskCore.UI.Pages.Persona.CreatePersona;
using Dimension.MaskCore.UI.Pages.Persona.RenamePersona;

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

    private void RenamePersona(UiPersonaModel item)
    {
        new RenamePersonaDialog(item).ShowAsync();
    }
}