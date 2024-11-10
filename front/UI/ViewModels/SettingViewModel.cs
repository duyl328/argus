using MaterialDesignThemes.Wpf;
using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using System.Collections.Generic;
using UI.Model;
using UI.Views;
using UI.Views.Setting;

namespace UI.ViewModels
{
    public class SettingViewModel : BindableBase
    {
        /// <summary>
        ///    Tab 展示列表
        /// </summary>
        public List<TabItem> TabItems { get; set; }

        private int _tabSelectedIndex = 0;

        /// <summary>
        ///     tab 按钮项选中的元素
        /// </summary>
        public int TabSelectedIndex
        {
            get => _tabSelectedIndex;
            set => SetProperty(ref _tabSelectedIndex, value);
        }

        private TabItem? _tabSelectedItem;

        /// <summary>
        ///     当前列表选中的元素
        /// </summary>
        public TabItem? TabSelectedItem
        {
            get => _tabSelectedItem;
            set => SetProperty(ref _tabSelectedItem, value);
        }

        /// <summary>
        ///     设置页面主要展示区域名称
        /// </summary>
        public static string SettingOperateRegionName => "SettingOperateRegionName";

        /// <summary>
        ///     tab 列表元素切换
        /// </summary>
        public DelegateCommand TabSelectChangedCommand { get; set; }

        /// <summary>
        ///     区域
        /// </summary>
        private IRegionManager RegionManager { get; }

        public SettingViewModel(
            IRegionManager regionManager,
            IEventAggregator eventAggregator)
        {
            TabItems = GetTabItems();
            RegionManager = regionManager;

            TabSelectChangedCommand = new DelegateCommand(TabSelectChanged);
            regionManager.RegisterViewWithRegion(SettingOperateRegionName, nameof(GeneralSettingView));
        }

        /// <summary>
        ///     tab 列表元素切换
        /// </summary>
        private void TabSelectChanged()
        {
            if (TabSelectedItem == null) return;
            if ("".Equals(TabSelectedItem.RegionName))
            {
                RegionManager.Regions[SettingOperateRegionName].RequestNavigate(nameof(NotBuildView));
                return;
            }

            RegionManager.Regions[SettingOperateRegionName].RequestNavigate(TabSelectedItem.RegionName);
        }

        private List<TabItem> GetTabItems()
        {
            // 一般；资料库；高级；
            return new()
            {
                new TabItem
                {
                    Title = "一般",
                    SelectedIcon = PackIconKind.ApplicationSettings,
                    UnselectedIcon = PackIconKind.ApplicationSettingsOutline,
                    ToolTipTitle = "一般",
                    RegionName = nameof(GeneralSettingView)
                },
                new TabItem
                {
                    Title = "资料库",
                    SelectedIcon = PackIconKind.Bookshelf,
                    UnselectedIcon = PackIconKind.Bookshelf,
                    ToolTipTitle = "资料库",
                    RegionName = nameof(LibrarySettingView)

                },
                new TabItem
                {
                    Title = "高级",
                    SelectedIcon = PackIconKind.WrenchCog,
                    UnselectedIcon = PackIconKind.WrenchCogOutline,
                    ToolTipTitle = "高级"
                },
            };
        }
    }
}
