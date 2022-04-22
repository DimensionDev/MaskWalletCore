using Avalonia;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using Dimension.MaskCore.Lifecycle.Controls;
using Dimension.MaskCore.UI.Pages.Persona.MnemonicValidate;

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

    private void NextClicked(object? sender, RoutedEventArgs e)
    {
        Navigate<MnemonicValidatePage>(new MnemonicValidateParameter(string.Join(" ", ViewModel.Mnemonic), ViewModel.Name));
    }
}