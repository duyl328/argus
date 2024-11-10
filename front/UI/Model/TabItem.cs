using MaterialDesignThemes.Wpf;
using Prism.Mvvm;
using UI.Constants;

namespace UI.Model
{
    public class TabItem : BindableBase
    {
        public string? Title { get; set; } = ConstUI.ListTitleNull;
        public string? ToolTipTitle { get; set; } = ConstUI.ListToolTipNull;
        public PackIconKind SelectedIcon { get; set; }
        public PackIconKind UnselectedIcon { get; set; }

        /// <summary>
        ///     要展示的区域名称
        /// </summary>
        public string RegionName { get; init; } = "";

        private object? _notification = null;

        public object? Notification
        {
            get => _notification;
            set => SetProperty(ref _notification, value);
        }
    }
}
