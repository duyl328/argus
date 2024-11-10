using Prism.Mvvm;
using System.Collections.ObjectModel;
using UI.ViewModels.Zipper;

namespace UI.ViewModels
{
    public class ZipperViewModel : BindableBase
    {
        private bool _isExpanded = true;

        /// <summary>
        ///     是否扩大
        /// </summary>
        public bool IsExpanded
        {
            get => _isExpanded;
            set => SetProperty(ref _isExpanded, value);
        }

        /// <summary>
        ///     压缩元素列表
        /// </summary>
        public ObservableCollection<FileSelectAdapter> SelectFilePathList { get; set; }

        public ZipperViewModel()
        {
            SelectFilePathList = GetSelectFilePathList();
        }

        /// <summary>
        ///     获取默认列表
        /// </summary>
        private ObservableCollection<FileSelectAdapter> GetSelectFilePathList()
        {
            return new ObservableCollection<FileSelectAdapter>()
            {
                new() { Path = "111" },
                new() { Path = "222" },
                new() { Path = "333" },
                new() { Path = "444" },
                new() { Path = "555" },
                new() { Path = "222" },
                new() { Path = "222" },
            };
        }
    }
}
