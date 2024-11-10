using HandyControl.Controls;
using MaterialDesignThemes.Wpf;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Windows;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using UI.ViewModels;

namespace UI.Views
{
    public partial class MainView : HandyControl.Controls.Window
    {
        private NotifyIcon notifyIcon;

        public MainView()
        {
            InitializeComponent();

            this.notifyIcon = new NotifyIcon();
            // this.notifyIcon.BalloonTipText = "Hello, NotifyIcon!";
            // notifyIcon.ShowBalloonTip("测试内容","",new );
            this.notifyIcon.Text = "Hello, NotifyIcon!";
            // this.notifyIcon.Icon = new System.Drawing.Icon("NotifyIcon.ico");
            this.notifyIcon.Visibility = Visibility.Visible;
        }

        /// <summary>
        ///     鼠标点击后拖动
        /// </summary>
        private void TipMouseMove(object sender, MouseEventArgs e)
        {
            if (e.LeftButton == MouseButtonState.Pressed)
            {
                if (WindowState == WindowState.Maximized)
                {
                    var point = PointToScreen(e.MouseDevice.GetPosition(this));
                    var width = RestoreBounds.Width;
                    var height = RestoreBounds.Height;
                    var x = point.X - width / 2;
                    var y = 0;
                    Left = x;
                    Top = y;
                    WindowState = WindowState.Normal;
                }
                DragMove();
            }
        }

    }

}
