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
    const string _path = "m/44'/60'/0'/0/0";
    const string _password = "";
    
    private readonly Realm _realm = Ioc.Default.GetRequiredService<Realm>();
    public IObservable<IReadOnlyCollection<UiPersonaModel>> Personas;

    public PersonaRepository()
    {
        Personas = _realm.All<DbPersonaModel>().AsObservable()
            .Select(it => it.Select(UiPersonaModel.FromDb).ToImmutableList());
    }

    public async Task CreatePersonaFromMnemonic(string mnemonic, string name)
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
            _password,
            _path,
            CurveType.Secp256k1,
            new EncryptionOption(EncryptionOption.EncVersion.V38)
        ));
        _realm.Write(() =>
        {
            _realm.Add(DbPersonaModel.FromPersona(persona, mnemonic, _path, _password, false, name));
        });
    }
}