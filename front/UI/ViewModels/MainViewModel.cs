using HandyControl.Controls;
using HandyControl.Interactivity;
using MaterialDesignThemes.Wpf;
using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Configuration;
using System.Windows;
using System.Windows.Input;
using System.Windows.Media.Animation;
using UI.Views;
using TabItem = UI.Model.TabItem;

namespace UI.ViewModels
{
    public class MainViewModel : BindableBase
    {
        #region prop

        private Visibility _iconTabMenuVisibility = Visibility.Visible;

        /// <summary>
        ///     单 icon 图标列表是否展示
        /// </summary>
        public Visibility IconTabMenuVisibility
        {
            get => _iconTabMenuVisibility;
            set => SetProperty(ref _iconTabMenuVisibility, value);
        }

        private bool _isOpenMenu = false;

        /// <summary>
        ///     菜单是否打开
        /// </summary>
        public bool IsOpenMenu
        {
            get => _isOpenMenu;
            set
            {
                IconTabMenuVisibility = value ? Visibility.Collapsed : Visibility.Visible;
                SetProperty(ref _isOpenMenu, value);
            }
        }

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

        #endregion

        #region attrib

        /// <summary>
        ///     主页面主要展示区域名称
        /// </summary>
        public static string MainOperateRegionName => "MainOperateRegionName";

        /// <summary>
        ///     区域
        /// </summary>
        private IRegionManager RegionManager { get; }

        /// <summary>
        ///    Tab 展示列表
        /// </summary>
        public List<TabItem> TabItems { get; set; }

        /// <summary>
        ///     去主页
        /// </summary>
        public DelegateCommand GoHomeCommand { get; set; }

        /// <summary>
        ///     菜单按钮点击
        /// </summary>
        public DelegateCommand MenuClickCommand { get; set; }

        /// <summary>
        ///     tab 列表元素切换
        /// </summary>
        public DelegateCommand TabSelectChangedCommand { get; set; }

        private void OnMouseDown(MouseEventArgs e)
        {
        }

        public ICommand eventToCommand { get; }

        /// <summary>
        ///     Event
        /// </summary>
        private readonly IEventAggregator _eventAggregator;

        #endregion

        public MainViewModel(
            IRegionManager regionManager,
            IEventAggregator eventAggregator)
        {
            eventToCommand = new DelegateCommand<MouseEventArgs>(OnMouseDown);

            _eventAggregator = eventAggregator;
            // 订阅事件
            SubEvent();
            RegionManager = regionManager;

            TabItems = GetTabItems();
            GoHomeCommand = new DelegateCommand(GoHome);
            MenuClickCommand = new DelegateCommand(MenuClick);

            TabSelectChangedCommand = new DelegateCommand(TabSelectChanged);

            // todo:默认使用 Home
            regionManager.RegisterViewWithRegion(MainOperateRegionName, nameof(ZipperView));
        }

        #region func

        /// <summary>
        ///     订阅通知
        /// </summary>
        private void SubEvent()
        {
            _eventAggregator.GetEvent<NoticeGoHome>().Subscribe(GoHome);
        }

        /// <summary>
        ///     tab 列表元素切换
        /// </summary>
        private void TabSelectChanged()
        {
            if (TabSelectedItem == null) return;
            if ("".Equals(TabSelectedItem.RegionName))
            {
                RegionManager.Regions[MainOperateRegionName].RequestNavigate(nameof(NotBuildView));
                return;
            }

            RegionManager.Regions[MainOperateRegionName].RequestNavigate(TabSelectedItem.RegionName);
        }

        /// <summary>
        ///     菜单按钮点击
        /// </summary>
        private void MenuClick()
        {
        }

        /// <summary>
        ///     获取展示列表
        /// </summary>
        private List<TabItem> GetTabItems()
        {
            return
            [
                new TabItem
                {
                    Title = "搜索",
                    SelectedIcon = PackIconKind.ToyBrickSearch,
                    UnselectedIcon = PackIconKind.ToyBrickSearchOutline,
                    RegionName = nameof(SearchView),
                    ToolTipTitle = "搜索"
                },
                new TabItem
                {
                    Title = "压缩包处理",
                    SelectedIcon = PackIconKind.FolderZip,
                    UnselectedIcon = PackIconKind.FolderZipOutline,
                    // Notification = 3,
                    RegionName = nameof(ZipperView),
                    ToolTipTitle = "压缩包处理"
                },

                new TabItem
                {
                    Title = "文件",
                    SelectedIcon = PackIconKind.Folder,
                    UnselectedIcon = PackIconKind.FolderOutline,
                },

                new TabItem
                {
                    Title = "资料库",
                    SelectedIcon = PackIconKind.Bookshelf,
                    UnselectedIcon = PackIconKind.Bookshelf,
                },

                new TabItem
                {
                    Title = "地图",
                    SelectedIcon = PackIconKind.MapMarkerRadius,
                    UnselectedIcon = PackIconKind.MapMarkerRadiusOutline,
                    ToolTipTitle = "地图",
                    RegionName = nameof(MapView),
                },

                new TabItem
                {
                    Title = "设置",
                    SelectedIcon = PackIconKind.Settings,
                    UnselectedIcon = PackIconKind.SettingsOutline,
                    RegionName = nameof(SettingView),
                    ToolTipTitle = "设置"
                }
            ];
        }

        /// <summary>
        ///     去主页
        /// </summary>
        private void GoHome()
        {
            TabSelectedIndex = -1;
            TabSelectedItem = null;
            var regionManagerRegion = RegionManager.Regions[MainOperateRegionName];
            regionManagerRegion.RequestNavigate(nameof(HomeView));
        }

        #endregion
    }

    /// <summary>
    ///     切换至 Home
    /// </summary>
    public class NoticeGoHome : PubSubEvent
    {
    }
}
