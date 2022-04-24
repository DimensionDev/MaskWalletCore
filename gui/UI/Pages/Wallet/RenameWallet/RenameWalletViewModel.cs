using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.DependencyInjection;
using CommunityToolkit.Mvvm.Input;
using Dimension.MaskCore.Data.Repository;
using Dimension.MaskCore.Lifecycle.ViewModel;
using Dimension.MaskCore.UI.Model;

namespace Dimension.MaskCore.UI.Pages.Wallet.RenameWallet;

internal partial class RenameWalletViewModel : ViewModel
{
    private readonly WalletRepository _repository = Ioc.Default.GetRequiredService<WalletRepository>();
    [ObservableProperty] private string _name = string.Empty;
    private UiWalletModel? _wallet;

    public void Initialize(UiWalletModel item)
    {
        Name = item.Name;
        _wallet = item;
    }

    [ICommand]
    private void Rename()
    {
        if (_wallet == null)
        {
            return;
        }

        _repository.RenameWallet(_wallet.Address, Name);
    }
}