using Dimension.MaskCore.Data.Model;

namespace Dimension.MaskCore.UI.Model;

internal record UiPersonaModel(string Name, string Identifier)
{
    public static UiPersonaModel FromDb(DbPersonaModel model) => new(model.Name, model.Identifier);
}

internal record MnemonicModel(string Word, int Index);
