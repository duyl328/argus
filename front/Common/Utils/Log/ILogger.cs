using Serilog.Events;
using System;

namespace Common.Utils.Log
{
    public interface ILogger
    {
        /// <summary>
        ///     初始化日志
        /// </summary>
        /// <param name="options"></param>
        void SetLoggerConfig(LoggerOptions options);

        void Debug(string msg, Exception? e = null);
        void Info(string msg, Exception? e = null);
        void Warn(string msg, Exception? e = null);
        void Error(string msg, Exception? e = null);
        void Fatal(string msg, Exception? e = null);
        void Verbose(string msg, Exception? e = null);

    }
}
