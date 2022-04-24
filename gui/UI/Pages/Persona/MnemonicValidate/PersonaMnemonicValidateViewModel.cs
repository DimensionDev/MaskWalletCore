using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Threading.Tasks;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.DependencyInjection;
using CommunityToolkit.Mvvm.Input;
using Dimension.MaskCore.Data.Repository;
using Dimension.MaskCore.Lifecycle.ViewModel;

namespace Dimension.MaskCore.UI.Pages.Persona.MnemonicValidate;

internal partial class PersonaMnemonicValidateViewModel : ParameterizedViewModel<PersonaMnemonicValidateParameter>
{
    private readonly PersonaRepository _personaRepository = Ioc.Default.GetRequiredService<PersonaRepository>();
    [ObservableProperty] private IReadOnlyCollection<string> _words = new List<string>();
    public Action? OnComplete { get; set; }
    public PersonaMnemonicValidateParameter? Parameter { get; private set; }

    protected override void InitializeCore(PersonaMnemonicValidateParameter parameter)
    {
        Parameter = parameter;
        Words = Parameter.WordList.ToImmutableList();
    }

    [ICommand]
    private async Task Confirm()
    {
        if (Parameter != null)
        {
            await _personaRepository.CreatePersona(Parameter.Name, Parameter.Mnemonic);
            OnComplete?.Invoke();
        }
    }
}

internal record PersonaMnemonicValidateParameter(string Mnemonic, string Name)
{
    public string[] WordList => Mnemonic.Split(' ');
}