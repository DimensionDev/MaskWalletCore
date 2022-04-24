using Avalonia.Interactivity;
using Dimension.MaskCore.Lifecycle.Controls;

namespace Dimension.MaskCore.UI.Pages.Persona.MnemonicValidate;

internal partial class
    PersonaMnemonicValidatePage : Page<PersonaMnemonicValidateViewModel, PersonaMnemonicValidateParameter>
{
    public PersonaMnemonicValidatePage()
    {
        InitializeComponent();
        ViewModel.OnComplete = OnComplete;
    }

    private async void OnComplete()
    {
        await new Dialog
        {
            Title = "Persona Created",
            CloseButtonText = "OK"
        }.ShowAsync();
        Navigate<PersonaPage>();
    }

    private void BackClicked(object? sender, RoutedEventArgs e)
    {
        GoBack();
    }
}