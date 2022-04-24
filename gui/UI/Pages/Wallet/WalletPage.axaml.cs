using Avalonia.Interactivity;
using Dimension.MaskCore.Lifecycle.Controls;
using Dimension.MaskCore.UI.Model;
using Dimension.MaskCore.UI.Pages.Wallet.CreateWallet;
using Dimension.MaskCore.UI.Pages.Wallet.RenameWallet;

namespace Dimension.MaskCore.UI.Pages.Wallet;

internal partial class WalletPage : Page<WalletViewModel>
{
    public WalletPage()
    {
        InitializeComponent();
    }

    private void New_OnClicked(object? sender, RoutedEventArgs e)
    {
        Navigate<CreateWalletPage>();
    }

    private void RenameWallet(UiWalletModel item)
    {
        new RenameWalletDialog(item).ShowAsync();
    }
}