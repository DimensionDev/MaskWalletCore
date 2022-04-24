using Dimension.MaskCore.Lifecycle.Controls;
using Dimension.MaskCore.UI.Model;

namespace Dimension.MaskCore.UI.Pages.Wallet.RenameWallet;

internal partial class RenameWalletDialog : Dialog<RenameWalletViewModel>
{
    public RenameWalletDialog()
    {
        InitializeComponent();
    }

    public RenameWalletDialog(UiWalletModel item)
    {
        InitializeComponent();
        ViewModel.Initialize(item);
    }
}