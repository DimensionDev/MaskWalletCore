using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.DependencyInjection;
using CommunityToolkit.Mvvm.Input;
using Dimension.MaskCore.Data.Repository;
using Dimension.MaskCore.Lifecycle.ViewModel;
using Dimension.MaskCore.UI.Model;

namespace Dimension.MaskCore.UI.Pages.Persona.RenamePersona;

internal partial class RenamePersonaViewModel : ViewModel
{
    private readonly PersonaRepository _repository = Ioc.Default.GetRequiredService<PersonaRepository>();
    [ObservableProperty] private string _name = string.Empty;
    private UiPersonaModel? _persona;

    public void Initialize(UiPersonaModel item)
    {
        Name = item.Name;
        _persona = item;
    }

    [ICommand]
    private void Rename()
    {
        if (_persona == null)
        {
            return;
        }

        _repository.UpdatePersonaName(_persona.Identifier, Name);
    }
}