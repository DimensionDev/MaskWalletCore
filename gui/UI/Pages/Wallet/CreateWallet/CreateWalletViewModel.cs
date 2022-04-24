using CommunityToolkit.Mvvm.ComponentModel;
using Dimension.MaskCore.Common.Helpers;
using Dimension.MaskCore.Lifecycle.ViewModel;

namespace Dimension.MaskCore.UI.Pages.Wallet.CreateWallet;

internal partial class CreateWalletViewModel : ViewModel
{
    [ObservableProperty] private string _name = string.Empty;

    public string[] Mnemonic { get; } = MnemonicHelper.GenerateMnemonic().Split(' ');
}