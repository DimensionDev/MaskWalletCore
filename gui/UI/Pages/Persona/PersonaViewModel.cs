using System;
using System.Collections.Generic;
using System.Reactive.Subjects;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.DependencyInjection;
using Dimension.MaskCore.Data.Repository;
using Dimension.MaskCore.Lifecycle.ViewModel;
using Dimension.MaskCore.UI.Model;

namespace Dimension.MaskCore.UI.Pages.Persona;

internal partial class PersonaViewModel : ViewModel
{
    private static readonly PersonaRepository _personaRepository = Ioc.Default.GetRequiredService<PersonaRepository>();
    private readonly BehaviorSubject<string> _searchTextSubject = new(string.Empty);
    [ObservableProperty] private string _searchText = string.Empty;
    
    partial void OnSearchTextChanged(string value)
    {
        _searchTextSubject.OnNext(value);
    }
    
    public IObservable<IReadOnlyCollection<UiPersonaModel>> Personas => _personaRepository.Personas;

}