using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Reactive.Linq;
using System.Threading.Tasks;
using CommunityToolkit.Mvvm.DependencyInjection;
using Dimension.MaskCore.Common.Extension;
using Dimension.MaskCore.Data.Model;
using Dimension.MaskCore.UI.Model;
using Dimension.MaskWalletCore;
using Realms;

namespace Dimension.MaskCore.Data.Repository;

internal class PersonaRepository
{
    private const string Path = "m/44'/60'/0'/0/0";
    private const string Password = "";

    private static readonly Realm _realm = Ioc.Default.GetRequiredService<Realm>();

    public IObservable<IReadOnlyCollection<UiPersonaModel>> Personas { get; } = _realm.All<DbPersonaModel>()
        .AsObservable()
        .Select(it => it.Select(UiPersonaModel.FromDb).ToImmutableList());

    public async Task CreatePersona(string name, string mnemonic)
    {
        if (string.IsNullOrEmpty(mnemonic) || string.IsNullOrEmpty(name))
        {
            throw new ArgumentNullException(nameof(mnemonic));
        }

        if (_realm.All<DbPersonaModel>().Any(it => it.Mnemonic == mnemonic))
        {
            throw new ArgumentException($"Persona with name {name} already exists");
        }

        var persona = await Task.Run(() => PersonaKey.Create(
            mnemonic,
            Password,
            Path,
            CurveType.Secp256k1,
            new EncryptionOption(EncryptionOption.EncVersion.V38)
        ));
        _realm.Write(() => { _realm.Add(DbPersonaModel.FromPersona(persona, mnemonic, Path, Password, false, name)); });
    }

    public void UpdatePersonaName(string identifier, string name)
    {
        var dbPersona = _realm.All<DbPersonaModel>().FirstOrDefault(it => it.Identifier == identifier);
        if (dbPersona == null)
        {
            return;
        }

        _realm.Write(() => { dbPersona.Name = name; });
    }

    public void DeletePersona(string identifier)
    {
        var item = _realm.All<DbPersonaModel>().FirstOrDefault(it => it.Identifier == identifier);
        if (item == null)
        {
            return;
        }

        _realm.Write(() => _realm.Remove(item));
    }
}