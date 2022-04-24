using Dimension.MaskCore.Lifecycle.Controls;
using Dimension.MaskCore.UI.Model;

namespace Dimension.MaskCore.UI.Pages.Persona.RenamePersona;

internal partial class RenamePersonaDialog : Dialog<RenamePersonaViewModel>
{
    public RenamePersonaDialog()
    {
        InitializeComponent();
    }

    public RenamePersonaDialog(UiPersonaModel item)
    {
        InitializeComponent();
        ViewModel.Initialize(item);
    }
}