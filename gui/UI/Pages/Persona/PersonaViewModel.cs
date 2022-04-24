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

namespace Dimension.MaskCore.UI.Pages.Persona;

internal partial class PersonaViewModel : ViewModel
{
    private readonly PersonaRepository _personaRepository = Ioc.Default.GetRequiredService<PersonaRepository>();
    private readonly BehaviorSubject<string> _searchTextSubject = new(string.Empty);
    [ObservableProperty] private string _searchText = string.Empty;

    public PersonaViewModel()
    {
        Personas = _personaRepository.Personas.CombineLatest(_searchTextSubject).Throttle(TimeSpan.FromSeconds(0.3))
            .Select(
                it =>
                {
                    var (personas, searchText) = it;
                    if (string.IsNullOrWhiteSpace(searchText))
                    {
                        return personas;
                    }

                    return personas.Where(persona =>
                        persona.Name.Contains(searchText, StringComparison.OrdinalIgnoreCase) ||
                        persona.Identifier.Contains(searchText, StringComparison.OrdinalIgnoreCase)).ToImmutableList();
                });
    }

    public IObservable<IReadOnlyCollection<UiPersonaModel>> Personas { get; }

    [ICommand]
    private void DeletePersona(UiPersonaModel model)
    {
        _personaRepository.DeletePersona(model.Identifier);
    }

    partial void OnSearchTextChanged(string value)
    {
        _searchTextSubject.OnNext(value);
    }
}