namespace Common.Utils.Log
{
    public class LoggerFacade
    {
        private static ILogger _logger;

        public static ILogger Logger => CreateLogger();

        /// <summary>
        ///     创建记录器
        /// </summary>
        /// <returns></returns>
        public static ILogger CreateLogger()
        {
            if (_logger == null)
            {
                _logger = new SerilogLogger();
            }
            return _logger;
        }
    }
}
