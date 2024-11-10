using Prism.Mvvm;
using Repository.entity;

namespace UI.ViewModels.Setting
{
    /// <summary>
    ///     PhotoStoragePath 页面视图模型
    /// </summary>
    public class PhotoStoragePathViewModel : BindableBase
    {
        private PhotoStoragePath _model;

        public PhotoStoragePathViewModel(PhotoStoragePath model)
        {
            _model = model;
        }

        public string StoragePath
        {
            get => _model.StoragePath;
            set
            {
                if (_model.StoragePath == value) return;
                _model.StoragePath = value;
                RaisePropertyChanged();
            }
        }

        public bool IsEnable
        {
            get => _model.IsEnable;
            set
            {
                if (_model.IsEnable == value) return;
                _model.IsEnable = value;
                RaisePropertyChanged();
            }
        }
    }
}
