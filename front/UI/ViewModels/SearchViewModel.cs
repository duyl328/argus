using System.Collections.ObjectModel;
using CefSharp.DevTools.FileSystem;
using Common.Utils;
using Common.Utils.Log;
using Framework.Tools;
using Microsoft.EntityFrameworkCore.ChangeTracking;
using Repository.entity;
using Repository.Servers;
using UI.ViewModels.Setting;
using Directory = System.IO.Directory;

namespace UI.ViewModels;

public class SearchViewModel : BaseModel, INavigationAware
{
    /// <summary>
    ///     图像列表
    /// </summary>
    public ObservableCollection<SearchImgShowFormat> ImgList { get; set; } =
        new ObservableCollectionListSource<SearchImgShowFormat>();


    /// <summary>
    ///     相册存储路径
    /// </summary>
    public ObservableCollection<PhotoStoragePathViewModel> PhotoStoragePathList { get; set; } =
        new ObservableCollectionListSource<PhotoStoragePathViewModel>();

    /// <summary>
    ///     获取数据库操作
    /// </summary>
    private readonly PhotoStoragePathServer
        _photoStoragePathServer = PhotoStoragePathServer.GetPhotoStoragePathServer();

    public void OnNavigatedTo(NavigationContext navigationContext)
    {
        // 放置初始化代码，例如加载数据
        InitializeData();
    }

    public bool IsNavigationTarget(NavigationContext navigationContext)
    {
        // 是否可以重复使用该页面的实例
        return true;
    }

    public void OnNavigatedFrom(NavigationContext navigationContext)
    {
    }

    private async void InitializeData()
    {
        LoggerFacade.Logger.Info($"页面渲染");
        ImgList.Clear();
        // 访问数据库，更改页面数据
        // 异步查询数据库
        var photoStoragePaths = await GetAllPhotoStoragePathsAsync();
        for (var i = 0; i < photoStoragePaths.Count; i++)
        {
            var photoStoragePath = photoStoragePaths[i];
            if (photoStoragePath.IsEnable)
            {
                var paths = Directory.GetFiles(photoStoragePath.StoragePath);
                for (var i1 = 0; i1 < paths.Length; i1++)
                {
                    var path = paths[i1];
                    if (FileTypeChecker.IsImageFile(path))
                    {
                        var a = App.Configuration["Logging"];
                        var s = ImgUtils.GetImageDimensions(path);
                        LoggerFacade.Logger.Info($"图像大小:{s.height} ,{s.width}, a:{a}");
                        ImgList.Add(new SearchImgShowFormat(path, path));
                    }
                }
            }
        }
        
        for (var i = 0; i < ImgList.Count; i++)
        {
            var searchImgShowFormat = ImgList[i];
            
            LoggerFacade.Logger.Info($"路径内容:{searchImgShowFormat.ImgPath}");
        }
       
        // 清空现有数据并添加新数据
        PhotoStoragePathList.Clear();
        foreach (var path in photoStoragePaths)
        {
            PhotoStoragePathList.Add(new PhotoStoragePathViewModel(path));
        }

        // 数据初始化逻辑
    }

    private async Task<List<PhotoStoragePath>> GetAllPhotoStoragePathsAsync()
    {
        return await Task.Run(() => _photoStoragePathServer.GetPhotoStoragePaths());
    }
}

/// <summary>
///     搜索页面文件展示格式
/// </summary>
public class SearchImgShowFormat
{
    /// <summary>
    ///     文件名称
    /// </summary>
    public string ImgName { get; set; }

    /// <summary>
    ///     文件地址
    /// </summary>
    public string ImgPath { get; set; }
    

    public SearchImgShowFormat(string imgName, string imgPath)
    {
        ImgName = imgName;
        ImgPath = imgPath;
    }
}
