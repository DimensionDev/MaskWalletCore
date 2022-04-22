using System.Collections.Generic;
using System.Linq;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Dimension.MaskCore.Common.Extension;
using Dimension.MaskCore.Lifecycle.ViewModel;
using Dimension.MaskCore.UI.Model;

namespace Dimension.MaskCore.UI.Pages.Persona.MnemonicValidate;

internal partial class MnemonicValidateViewModel : ViewModel
{
    private readonly List<string> _mnemonic = new();
    [ObservableProperty]
    private string _name = string.Empty;
    [ObservableProperty]
    [AlsoNotifyCanExecuteFor(nameof(ComfirmCommand))]
    private IReadOnlyCollection<MnemonicModel> _selectedMnemonic = new List<MnemonicModel>();
    [ObservableProperty]
    [AlsoNotifyCanExecuteFor(nameof(ComfirmCommand))]
    private IReadOnlyCollection<MnemonicModel> _randomMnemonic = new List<MnemonicModel>();
    public void Init(MnemonicValidateParameter parameter)
    {
        _mnemonic.Clear();
        _mnemonic.AddRange(parameter.Mnemonic.Split(" "));
        Name = parameter.Name;
        RandomMnemonic = _mnemonic.Randomize().Select((word, index) => new MnemonicModel(word, index)).ToList();
    }

    [ICommand]
    private void SelectWord(MnemonicModel item)
    {
        if (SelectedMnemonic.Contains(item))
        {
            SelectedMnemonic = SelectedMnemonic.Where(x => x.Index != item.Index).ToList();
            RandomMnemonic = RandomMnemonic.Concat(new[] { item }).OrderBy(it => it.Index).ToList();
        }
        else
        {
            SelectedMnemonic = SelectedMnemonic.Concat(new[] { item }).ToList();
            RandomMnemonic = RandomMnemonic.Where(x => x.Index != item.Index).OrderBy(it => it.Index).ToList();
        }
    }
    
    public bool IsValid => SelectedMnemonic.Select(x => x.Word).SequenceEqual(_mnemonic);
    
    [ICommand(CanExecute = nameof(IsValid))]
    private void Comfirm()
    {
    }
}

internal record MnemonicValidateParameter(string Mnemonic, string Name);