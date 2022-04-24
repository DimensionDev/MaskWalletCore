using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Reactive.Linq;
using System.Threading.Tasks;
using CommunityToolkit.Mvvm.DependencyInjection;
using Dimension.MaskCore.Common.Extension;
using Dimension.MaskCore.Data.Model;
using Dimension.MaskCore.Model;
using Dimension.MaskCore.UI.Model;
using Dimension.MaskWalletCore;
using Realms;

namespace Dimension.MaskCore.Data.Repository;

internal class WalletRepository
{
    private const string Path = "m/44'/60'/0'/0/0";
    private const string Password = "";

    private static readonly Realm _realm = Ioc.Default.GetRequiredService<Realm>();

    public IObservable<IReadOnlyCollection<UiWalletModel>> Wallets { get; } = _realm.All<DbWalletModel>()
        .AsObservable()
        .Select(list => list.Select(UiWalletModel.From).ToImmutableList());

    public async Task CreateWallet(string name, string mnemonic, string password = Password)
    {
        var wallet = await Task.Run(() => WalletKey.FromMnemonic(mnemonic, password));
        var account = await Task.Run(() => wallet.AddNewAccountAtPath(CoinType.Ethereum, Path, name, password));
        if (_realm.All<DbWalletModel>().Any(it => it.Address == account.Address))
        {
            return;
        }

        var item = new DbWalletModel
        {
            Name = name,
            Address = account.Address,
            DerivationPath = account.DerivationPath,
            Data = wallet.Data,
            PlatformType = PlatformType.Ethereum
        };
        _realm.Write(() => _realm.Add(item));
    }

    public void RenameWallet(string address, string name)
    {
        var item = _realm.All<DbWalletModel>().FirstOrDefault(it => it.Address == address);
        if (item == null)
        {
            return;
        }

        _realm.Write(() => item.Name = name);
    }

    public void DeleteWallet(string address)
    {
        var item = _realm.All<DbWalletModel>().FirstOrDefault(it => it.Address == address);
        if (item == null)
        {
            return;
        }

        _realm.Write(() => _realm.Remove(item));
    }
}