using System.IO;

namespace UI.ViewModels
{
    public class MapViewModel : BindableBase
    {
        private string _showHtmlPath = "";

        /// <summary>
        ///     html 文件地址
        /// </summary>
        public string ShowHtmlPath
        {
            get => _showHtmlPath;
            set => SetProperty(ref _showHtmlPath, value);
        }


        public MapViewModel(
            IRegionManager regionManager,
            IEventAggregator eventAggregator)
        {
            ShowHtmlPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "Htmls/Index.html");
        }
    }
}
