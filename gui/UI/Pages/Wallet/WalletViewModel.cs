using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Reactive.Linq;
using System.Reactive.Subjects;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.DependencyInjection;
using CommunityToolkit.Mvvm.Input;
using Dimension.MaskCore.Data.Repository;
using Dimension.MaskCore.Lifecycle.ViewModel;
using Dimension.MaskCore.UI.Model;
using TextCopy;

namespace Dimension.MaskCore.UI.Pages.Wallet;

internal partial class WalletViewModel : ViewModel
{
    private readonly WalletRepository _repository = Ioc.Default.GetRequiredService<WalletRepository>();
    private readonly BehaviorSubject<string> _searchTextSubject = new(string.Empty);
    [ObservableProperty] private string _searchText = string.Empty;

    public WalletViewModel()
    {
        Wallets = _repository.Wallets.CombineLatest(_searchTextSubject).Throttle(TimeSpan.FromSeconds(0.3)).Select(
            it =>
            {
                var (wallets, searchText) = it;
                if (string.IsNullOrWhiteSpace(searchText))
                {
                    return wallets;
                }

                return wallets.Where(wallet =>
                    wallet.Name.Contains(searchText, StringComparison.OrdinalIgnoreCase) ||
                    wallet.Address.Contains(searchText, StringComparison.OrdinalIgnoreCase)).ToImmutableList();
            });
    }

    public IObservable<IReadOnlyCollection<UiWalletModel>> Wallets { get; }

    [ICommand]
    private void DeleteWallet(UiWalletModel model)
    {
        _repository.DeleteWallet(model.Address);
    }

    [ICommand]
    private void CopyAddress(UiWalletModel model)
    {
        ClipboardService.SetText(model.Address);
    }

    partial void OnSearchTextChanged(string value)
    {
        _searchTextSubject.OnNext(value);
    }
}