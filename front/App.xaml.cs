using Framework.Utils.Log;
using Prism.Commands;
using Prism.DryIoc;
using Prism.Ioc;
using System;
using System.Collections.Generic;
using System.Configuration;
using System.Data;
using System.Linq;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Threading;
using UI.Views;
using UI.Views.Setting;

namespace UI
{
    /// <summary>
    /// Interaction logic for App.xaml
    /// </summary>
    public partial class App : PrismApplication
    {
        protected override void OnStartup(StartupEventArgs e)
        {
            ExceptionCatch();
            base.OnStartup(e);
            //UI.Resource.Language.

            LoggerFacade.Logger.Info("====================================== 程序启动! ==================================");
        }

        protected override void OnExit(ExitEventArgs e)
        {
            LoggerFacade.Logger.Info("--------------------------------------- 应用程序关闭! ------------------------------");

            base.OnExit(e);
        }

        /// <summary>
        ///     异常捕捉
        /// </summary>
        private void ExceptionCatch()
        {
            Current.DispatcherUnhandledException += App_OnDispatcherUnhandledException;
            AppDomain.CurrentDomain.UnhandledException += CurrentDomain_UnhandledException;
        }

        private void App_OnDispatcherUnhandledException(object sender, DispatcherUnhandledExceptionEventArgs e)
        {
            HandleException(e.Exception);
            e.Handled = true;
        }


        private void CurrentDomain_UnhandledException(object sender, UnhandledExceptionEventArgs e)
        {
            Exception? ex = e.ExceptionObject as Exception;
            HandleException(ex);
        }

        /// <summary>
        ///     处理异常
        /// </summary>
        /// <param name="exception"></param>
        private void HandleException(Exception? exception)
        {
            LoggerFacade.Logger.Error("未捕获异常", exception);
            // 错误记录，错误处理
            if (exception != null)
            {
                // todo:2023年10月29日 恭喜你发现了BUG页面
                MessageBox.Show($"恭喜你发现了BUG，请联系管理员解决：{exception.Message}");
            }
        }


        protected override void RegisterTypes(IContainerRegistry containerRegistry)
        {
            containerRegistry.RegisterForNavigation<HomeView>();
            containerRegistry.RegisterForNavigation<ZipperView>();
            containerRegistry.RegisterForNavigation<GeneralSettingView>();
            containerRegistry.RegisterForNavigation<SettingView>();
            containerRegistry.RegisterForNavigation<MapView>();
            containerRegistry.RegisterForNavigation<LibrarySettingView>();
            containerRegistry.RegisterForNavigation<NotBuildView>();
        }

        protected override Window CreateShell()
        {
            return Container.Resolve<MainView>();
        }
    }
}
