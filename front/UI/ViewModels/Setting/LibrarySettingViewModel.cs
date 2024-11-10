using Common.Utils.Log;
using HandyControl.Controls;
using Microsoft.EntityFrameworkCore.ChangeTracking;
using Ookii.Dialogs.Wpf;
using Prism.Commands;
using Prism.Mvvm;
using Repository.Contexts;
using Repository.entity;
using Repository.Servers;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Threading.Tasks;
using System.Windows.Input;

namespace UI.ViewModels.Setting
{
    public class LibrarySettingViewModel : BindableBase
    {
        /// <summary>
        ///     路径输入框获取焦点触发
        /// </summary>
        public ICommand PathSelectClick { get; }

        /// <summary>
        ///     鼠标左键点击【更新状态】
        /// </summary>
        public DelegateCommand<PhotoStoragePathViewModel> EnableChanged { get; }

        public ICommand PathSaveClick { get; }

        private string _inputPath = "";

        /// <summary>
        ///     输入框中填写的路径
        /// </summary>
        public string InputPath
        {
            get => _inputPath;
            set => SetProperty(ref _inputPath, value);
        }

        /// <summary>
        ///     获取数据库操作
        /// </summary>
        private readonly PhotoStoragePathServer
            _photoStoragePathServer = PhotoStoragePathServer.GetPhotoStoragePathServer();

        /// <summary>
        ///     相册存储路径
        /// </summary>
        public ObservableCollection<PhotoStoragePathViewModel> PhotoStoragePathList { get; set; } =
            new ObservableCollectionListSource<PhotoStoragePathViewModel>();

        public LibrarySettingViewModel()
        {
            PathSelectClick = new DelegateCommand(OnPathSelectCLick);
            PathSaveClick = new DelegateCommand(OnPathSaveClick);
            EnableChanged = new DelegateCommand<PhotoStoragePathViewModel>(OnEnableChanged);
            // 更新 UI，绑定数据
            LoadDataAsync();
        }

        /// <summary>
        ///     鼠标左键点击选中或反选
        /// </summary>
        private void OnEnableChanged(PhotoStoragePathViewModel parameter)
        {
            if (parameter is PhotoStoragePathViewModel data)
            {
                data.IsEnable = !data.IsEnable;
            }

            // 更新数据库状态
            _photoStoragePathServer.UpdatePhotoStorageState(parameter.StoragePath, parameter.IsEnable);
        }

        /// <summary>
        ///     异步更新数据
        /// </summary>
        private async void LoadDataAsync()
        {
            // 异步查询数据库
            var photoStoragePaths = await GetAllPhotoStoragePathsAsync();

            // 清空现有数据并添加新数据
            PhotoStoragePathList.Clear();
            foreach (var path in photoStoragePaths)
            {
                PhotoStoragePathList.Add(new PhotoStoragePathViewModel(path));
            }
        }

        /// <summary>
        ///     路径保存
        /// </summary>
        private void OnPathSaveClick()
        {
            // 如果数据为空
            if (String.IsNullOrWhiteSpace(InputPath))
            {
                LoggerFacade.Logger.Info("不能保存空路径！");

                Growl.Info("不能保存空路径!");
                return;
            }


            try
            {
                _photoStoragePathServer.AddPhotoStoragePath(new PhotoStoragePath()
                {
                    StoragePath = InputPath,
                    IsEnable = true
                });
                LoadDataAsync();
            }
            catch (Exception e)
            {
                LoggerFacade.Logger.Info("路径保存出错！", e);
                Growl.Info($"保存出错, {e.Message} !");
            }
        }

        /// <summary>
        ///     异步获取数据
        /// </summary>
        public async Task<List<PhotoStoragePath>> GetAllPhotoStoragePathsAsync()
        {
            // 使用 Task.Run 以在后台线程中执行数据库查询
            return await Task.Run(() => _photoStoragePathServer.GetPhotoStoragePaths());
        }

        /// <summary>
        ///     用户选择路径
        /// </summary>
        private void OnPathSelectCLick()
        {
            // 创建对话框
            VistaFolderBrowserDialog dialog = new VistaFolderBrowserDialog();

            // 显示对话框
            if (dialog.ShowDialog() == true)
            {
                // 获取选择的文件夹路径
                string selectedPath = dialog.SelectedPath;
                LoggerFacade.Logger.Info($"选择的文件夹路径是: {selectedPath}");
                InputPath = selectedPath;
            }
        }
    }
}
