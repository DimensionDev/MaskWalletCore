using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Threading.Tasks;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.DependencyInjection;
using CommunityToolkit.Mvvm.Input;
using Dimension.MaskCore.Data.Repository;
using Dimension.MaskCore.Lifecycle.ViewModel;

namespace Dimension.MaskCore.UI.Pages.Wallet.MnemonicValidate;

internal partial class WalletMnemonicValidateViewModel : ParameterizedViewModel<WalletMnemonicValidateParameter>
{
    private readonly WalletRepository _repository = Ioc.Default.GetRequiredService<WalletRepository>();
    [ObservableProperty] private IReadOnlyCollection<string> _words = new List<string>();
    public Action? OnComplete { get; set; }
    public WalletMnemonicValidateParameter? Parameter { get; private set; }

    protected override void InitializeCore(WalletMnemonicValidateParameter parameter)
    {
        Parameter = parameter;
        Words = Parameter.WordList.ToImmutableList();
    }

    [ICommand]
    private async Task Confirm()
    {
        if (Parameter != null)
        {
            await _repository.CreateWallet(Parameter.Name, Parameter.Mnemonic);
            OnComplete?.Invoke();
        }
    }
}

internal record WalletMnemonicValidateParameter(string Mnemonic, string Name)
{
    public string[] WordList => Mnemonic.Split(' ');
}