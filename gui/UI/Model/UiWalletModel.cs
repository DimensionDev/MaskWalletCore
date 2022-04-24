using Dimension.MaskCore.Data.Model;

namespace Dimension.MaskCore.UI.Model;

internal record UiWalletModel(string Address, string Name)
{
    public static UiWalletModel From(DbWalletModel item)
    {
        return new UiWalletModel(item.Address, item.Name);
    }
}