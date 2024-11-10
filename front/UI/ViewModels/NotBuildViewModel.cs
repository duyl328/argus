using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using UI.Views;

namespace UI.ViewModels
{
    public class NotBuildViewModel : BindableBase
    {
        /// <summary>
        ///     区域
        /// </summary>
        private IRegionManager RegionManager { get; }

        public DelegateCommand GoHomeCommand { get; set; }

        private readonly IEventAggregator _eventAggregator;

        public NotBuildViewModel(IRegionManager regionManager, IEventAggregator eventAggregator)
        {
            _eventAggregator = eventAggregator;
            RegionManager = regionManager;
            GoHomeCommand = new DelegateCommand(GoHome);
        }

        /// <summary>
        ///     回到主页
        /// </summary>
        private void GoHome()
        {
            _eventAggregator.GetEvent<NoticeGoHome>().Publish();
        }
    }
}
