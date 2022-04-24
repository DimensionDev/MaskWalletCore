using Avalonia.Interactivity;
using Dimension.MaskCore.Lifecycle.Controls;

namespace Dimension.MaskCore.UI.Pages.Wallet.MnemonicValidate;

internal partial class
    WalletMnemonicValidatePage : Page<WalletMnemonicValidateViewModel, WalletMnemonicValidateParameter>
{
    public WalletMnemonicValidatePage()
    {
        InitializeComponent();
        ViewModel.OnComplete = OnComplete;
    }

    private async void OnComplete()
    {
        await new Dialog
        {
            Title = "Wallet Created",
            CloseButtonText = "OK"
        }.ShowAsync();
        Navigate<WalletPage>();
    }

    private void BackClicked(object? sender, RoutedEventArgs e)
    {
        GoBack();
    }
}