using Serilog;
using Serilog.Events;
using System;

namespace Common.Utils.Log
{
    public class LoggerOptions
    {
        /// <summary>
        ///     日志等级
        /// </summary>
        public LogEventLevel MinimumLogLevel { get; set; } = LogEventLevel.Debug;

        /// <summary>
        ///     输出路径
        /// </summary>
        // public string LogFilePath { get; set; } = $"logs\\{DateTime.Now:yyyy-MM-dd}-.log";
        public string LogFilePath { get; set; } = $"logs\\log-.log";

        /// <summary>
        ///     日志文件大小限制【10兆】
        /// </summary>
        public int LogFileSize { get; set; } = 10485760;

        /// <summary>
        ///     文件保留时长【默认 7 天】
        /// </summary>
        public int FileRetainedTime { get; set; } = 7;

        /// <summary>
        ///     分类方式【按天分类】
        /// </summary>
        public RollingInterval RollingInterval { get; set; } = RollingInterval.Day;

        /// <summary>
        ///     输出模板
        /// </summary>
        public string OutputTemplate { get; set; } =
            "{Timestamp:yyyy-MM-dd HH:mm:ss.fff zzz} [{Level:u3}] {Message:lj}{NewLine}{Exception}";
    }
}
