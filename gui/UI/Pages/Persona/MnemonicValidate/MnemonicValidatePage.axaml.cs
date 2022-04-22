using Avalonia.Interactivity;
using Dimension.MaskCore.Lifecycle.Controls;

namespace Dimension.MaskCore.UI.Pages.Persona.MnemonicValidate;

internal partial class MnemonicValidatePage : Page<MnemonicValidateViewModel, MnemonicValidateParameter>
{
    public MnemonicValidatePage()
    {
        InitializeComponent();
    }

    protected override void OnCreated(MnemonicValidateParameter parameter)
    {
        base.OnCreated(parameter);
        ViewModel.Init(parameter);
    }

    private void BackClicked(object? sender, RoutedEventArgs e)
    {
        GoBack();
    }
}