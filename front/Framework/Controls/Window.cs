using Framework.Tools;
using System;
using System.Windows;
using System.Windows.Data;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Shell;

namespace Framework.Controls
{
    public class Window : System.Windows.Window
    {
        public static readonly DependencyProperty NonClientAreaHeightProperty = DependencyProperty.Register(
            nameof(NonClientAreaHeight), typeof(double), typeof(Window),
            new PropertyMetadata(22.0));

        public static readonly DependencyProperty ShowNonClientAreaProperty = DependencyProperty.Register(
            nameof(ShowNonClientArea), typeof(bool), typeof(Window),
            new PropertyMetadata(true, OnShowNonClientAreaChanged));

        public bool ShowNonClientArea
        {
            get => (bool)GetValue(ShowNonClientAreaProperty);
            set => SetValue(ShowNonClientAreaProperty, value);
        }

        public double NonClientAreaHeight
        {
            get => (double)GetValue(NonClientAreaHeightProperty);
            set => SetValue(NonClientAreaHeightProperty, value);
        }

        public static readonly DependencyProperty IsFullScreenProperty = DependencyProperty.Register(
            nameof(IsFullScreen), typeof(bool), typeof(Window),
            new PropertyMetadata(false, OnIsFullScreenChanged));

        public bool IsFullScreen
        {
            get => (bool)GetValue(IsFullScreenProperty);
            set => SetValue(IsFullScreenProperty, value);
        }

        private readonly Thickness _commonPadding;
        private Thickness _actualBorderThickness;
        private double _tempNonClientAreaHeight;
        private WindowState _tempWindowState;
        private WindowStyle _tempWindowStyle;

        private ResizeMode _tempResizeMode;

        private bool _showNonClientArea = true;
        private bool _isFullScreen;
        private UIElement _nonClientArea;

        public Window()
        {
            var chrome = new WindowChrome
            {
                // 窗口的边框圆角
                CornerRadius = new CornerRadius(),
                // 定义玻璃效果的边框厚度
                GlassFrameThickness = new Thickness(0, 0, 0, 1),
                // 禁用 Aero 样式的窗口标题栏按钮
                UseAeroCaptionButtons = false
            };
            // 使用BindingOperations为WindowChrome的CaptionHeight属性绑定NonClientAreaHeight属性
            BindingOperations.SetBinding(chrome, WindowChrome.CaptionHeightProperty,
                new Binding(NonClientAreaHeightProperty.Name) { Source = this });

            WindowChrome.SetWindowChrome(this, chrome);
            _commonPadding = Padding;

            Loaded += (s, e) => OnLoaded(e);
        }

        protected void OnLoaded(RoutedEventArgs args)
        {
            // 实际边框厚度和非客户区域高度
            // 可能用于后面进行窗口状态的恢复
            _actualBorderThickness = BorderThickness;
            _tempNonClientAreaHeight = NonClientAreaHeight;

            if (WindowState == WindowState.Maximized)
            {
                // 将窗口的边框厚度设置为空
                BorderThickness = new Thickness();
                // 增加非客户区域高度
                _tempNonClientAreaHeight += 8;
            }

            CommandBindings.Add(new CommandBinding(SystemCommands.MinimizeWindowCommand,
                (s, e) => WindowState = WindowState.Minimized));
            CommandBindings.Add(new CommandBinding(SystemCommands.MaximizeWindowCommand,
                (s, e) => WindowState = WindowState.Maximized));
            CommandBindings.Add(new CommandBinding(SystemCommands.RestoreWindowCommand,
                (s, e) => WindowState = WindowState.Normal));
            CommandBindings.Add(new CommandBinding(SystemCommands.CloseWindowCommand, (s, e) => Close()));
            CommandBindings.Add(new CommandBinding(SystemCommands.ShowSystemMenuCommand, ShowSystemMenu));

            _tempWindowState = WindowState;
            _tempWindowStyle = WindowStyle;
            _tempResizeMode = ResizeMode;

            SwitchIsFullScreen(_isFullScreen);
            SwitchShowNonClientArea(_showNonClientArea);

            if (WindowState == WindowState.Maximized)
            {
                _tempNonClientAreaHeight -= 8;
            }

            if (SizeToContent != SizeToContent.WidthAndHeight)
                return;

            SizeToContent = SizeToContent.Height;
            Dispatcher.BeginInvoke(new Action(() => { SizeToContent = SizeToContent.WidthAndHeight; }));
        }

        private static void OnIsFullScreenChanged(DependencyObject d, DependencyPropertyChangedEventArgs e)
        {
            var ctl = (Window)d;
            ctl.SwitchIsFullScreen((bool)e.NewValue);
        }

        private static void OnShowNonClientAreaChanged(DependencyObject d, DependencyPropertyChangedEventArgs e)
        {
            var ctl = (Window)d;
            ctl.SwitchShowNonClientArea((bool)e.NewValue);
        }

        private void SwitchShowNonClientArea(bool showNonClientArea)
        {
            if (_nonClientArea == null)
            {
                _showNonClientArea = showNonClientArea;
                return;
            }

            if (showNonClientArea)
            {
                if (IsFullScreen)
                {
                    _nonClientArea.Show(false);
                    _tempNonClientAreaHeight = NonClientAreaHeight;
                    NonClientAreaHeight = 0;
                }
                else
                {
                    _nonClientArea.Show(true);
                    NonClientAreaHeight = _tempNonClientAreaHeight;
                }
            }
            else
            {
                _nonClientArea.Show(false);
                _tempNonClientAreaHeight = NonClientAreaHeight;
                NonClientAreaHeight = 0;
            }
        }

        private void SwitchIsFullScreen(bool isFullScreen)
        {
            if (_nonClientArea == null)
            {
                _isFullScreen = isFullScreen;
                return;
            }

            if (isFullScreen)
            {
                _nonClientArea.Show(false);
                _tempNonClientAreaHeight = NonClientAreaHeight;
                NonClientAreaHeight = 0;

                _tempWindowState = WindowState;
                _tempWindowStyle = WindowStyle;
                _tempResizeMode = ResizeMode;
                WindowStyle = WindowStyle.None;
                //下面三行不能改变，就是故意的
                WindowState = WindowState.Maximized;
                WindowState = WindowState.Minimized;
                WindowState = WindowState.Maximized;
            }
            else
            {
                if (ShowNonClientArea)
                {
                    _nonClientArea.Show(true);
                    NonClientAreaHeight = _tempNonClientAreaHeight;
                }
                else
                {
                    _nonClientArea.Show(false);
                    _tempNonClientAreaHeight = NonClientAreaHeight;
                    NonClientAreaHeight = 0;
                }

                WindowState = _tempWindowState;
                WindowStyle = _tempWindowStyle;
                ResizeMode = _tempResizeMode;
            }
        }

        /// <summary>
        ///     展示系统菜单
        /// </summary>
        private void ShowSystemMenu(object sender, ExecutedRoutedEventArgs e)
        {
            var point = WindowState == WindowState.Maximized
                ? new Point(0, NonClientAreaHeight)
                : new Point(Left, Top + NonClientAreaHeight);
            SystemCommands.ShowSystemMenu(this, point);
        }
    }
}
