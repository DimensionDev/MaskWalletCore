using Avalonia.Interactivity;
using Dimension.MaskCore.Lifecycle.Controls;
using Dimension.MaskCore.UI.Pages.Wallet.MnemonicValidate;

namespace Dimension.MaskCore.UI.Pages.Wallet.CreateWallet;

internal partial class CreateWalletPage : Page<CreateWalletViewModel>
{
    public CreateWalletPage()
    {
        InitializeComponent();
    }

    private void BackClicked(object? sender, RoutedEventArgs e)
    {
        GoBack();
    }

    private void NextClicked(object? sender, RoutedEventArgs e)
    {
        Navigate<WalletMnemonicValidatePage>(new WalletMnemonicValidateParameter(
                string.Join(" ", ViewModel.Mnemonic),
                ViewModel.Name
            )
        );
    }
}