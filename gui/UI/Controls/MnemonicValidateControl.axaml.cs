using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Windows.Input;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Dimension.MaskCore.Common.Extension;
using Dimension.MaskCore.UI.Model;
using DynamicData;

namespace Dimension.MaskCore.UI.Controls;

internal partial class MnemonicValidateControl : UserControl
{
    public static readonly DirectProperty<MnemonicValidateControl, ICommand?> ConfirmCommandProperty =
        AvaloniaProperty.RegisterDirect<MnemonicValidateControl, ICommand?>(
            nameof(ConfirmCommand), o => o.ConfirmCommand, (o, v) => o.ConfirmCommand = v);

    public static readonly DirectProperty<MnemonicValidateControl, IReadOnlyCollection<string>> MnemonicWordsProperty =
        AvaloniaProperty.RegisterDirect<MnemonicValidateControl, IReadOnlyCollection<string>>(
            nameof(MnemonicWords), o => o.MnemonicWords, (o, v) => o.MnemonicWords = v);

    public static readonly DirectProperty<MnemonicValidateControl, bool> IsValidProperty =
        AvaloniaProperty.RegisterDirect<MnemonicValidateControl, bool>(
            nameof(IsValid),
            o => o.IsValid);

    private ICommand? _confirmCommand;
    private bool _isValid;

    private IReadOnlyCollection<string> _mnemonicWords = new List<string>();

    public MnemonicValidateControl()
    {
        InitializeComponent();
    }

    public ICommand? ConfirmCommand
    {
        get => _confirmCommand;
        set => SetAndRaise(ConfirmCommandProperty, ref _confirmCommand, value);
    }

    public IReadOnlyCollection<string> MnemonicWords
    {
        get => _mnemonicWords;
        set
        {
            SetAndRaise(MnemonicWordsProperty, ref _mnemonicWords, value);
            SelectedMnemonic.Clear();
            RandomMnemonic.Clear();
            RandomMnemonic.AddRange(value
                .Randomize()
                .Select((word, index) => new MnemonicModel(word, index)));
        }
    }

    public ObservableCollection<MnemonicModel> SelectedMnemonic { get; set; } = new();
    public ObservableCollection<MnemonicModel> RandomMnemonic { get; set; } = new();

    public bool IsValid
    {
        get => _isValid;
        private set => SetAndRaise(IsValidProperty, ref _isValid, value);
    }

    private void CheckValid()
    {
        IsValid = SelectedMnemonic.Select(x => x.Word).SequenceEqual(MnemonicWords);
    }

    private void ClearClicked(object? sender, RoutedEventArgs e)
    {
        SelectedMnemonic.Clear();
        RandomMnemonic.AddRange(MnemonicWords.Select((word, index) => new MnemonicModel(word, index)));
        CheckValid();
    }

    private void AddWord(MnemonicModel item)
    {
        if (!RandomMnemonic.Contains(item))
        {
            return;
        }

        RandomMnemonic.Remove(item);
        SelectedMnemonic.Add(item);
        CheckValid();
    }

    private void RemoveWord(MnemonicModel item)
    {
        if (!SelectedMnemonic.Contains(item))
        {
            return;
        }

        SelectedMnemonic.Remove(item);
        RandomMnemonic.Add(item);
        CheckValid();
    }

    public event EventHandler? Confirm;

    private void OnConfirmClicked(object? sender, RoutedEventArgs e)
    {
        Confirm?.Invoke(this, EventArgs.Empty);
    }
}