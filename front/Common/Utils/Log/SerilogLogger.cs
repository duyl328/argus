using Serilog;
using Serilog.Events;
using System;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Threading;

namespace Common.Utils.Log
{
    public class SerilogLogger : ILogger
    {
        /// <summary>
        ///     日志
        /// </summary>
        private Serilog.ILogger _logger;

        public SerilogLogger()
        {
            Serilog.Log.Logger = new LoggerConfiguration()
                .Enrich.FromLogContext()
                .MinimumLevel.Is(LogEventLevel.Debug)
                .WriteTo.Console()
                .WriteTo.File(
                    $"logs\\log-.log", // 日志文件路径
                    rollingInterval: RollingInterval.Day, // 按天分类
                    retainedFileCountLimit: 7, // 保留最近7天的日志文件
                    rollOnFileSizeLimit: true, // 当日志文件大小超过限制时滚动
                    fileSizeLimitBytes: 10485760, // 日志文件大小限制
                    shared: true) // 允许多个应用程序实例共享同一日志文件
                                  // 其他适配器和配置
                .CreateLogger();

            _logger = Serilog.Log.Logger;

        }

        /// <summary>
        ///     根据传入的配置重新获取日志打印器
        /// </summary>
        /// <param name="options"></param>
        public void SetLoggerConfig(LoggerOptions options)
        {
            Serilog.Log.Logger = new LoggerConfiguration()
                .Enrich.FromLogContext()
                .MinimumLevel.Is(options.MinimumLogLevel)
                .WriteTo.Console()
                .WriteTo.File(
                    options.LogFilePath,
                    rollingInterval: options.RollingInterval,
                    retainedFileCountLimit: options.FileRetainedTime,
                    rollOnFileSizeLimit: true,
                    fileSizeLimitBytes: options.LogFileSize,
                    outputTemplate: options.OutputTemplate,
                    shared: true)
                .CreateLogger();

            // 重新赋值
            _logger = Serilog.Log.Logger;
        }

        /// <summary>
        ///     获取当前执行的是哪个线程
        /// </summary>
        /// <returns></returns>
        private string GetThread()
        {
            return Thread.CurrentThread.Name ?? (Thread.CurrentThread.ManagedThreadId.Equals(1)
                ? "main"
                : $"Thread-{Thread.CurrentThread.ManagedThreadId}");
        }

        public void Debug(string msg, Exception? e)
        {
            _logger.Debug(e, "[{Thread}] {Msg}", GetThread(), msg);
        }

        public void Info(string msg, Exception? e)
        {
            _logger.Information(e, "[{Thread}] {Msg}", GetThread(), msg);
        }

        public void Warn(string msg, Exception? e)
        {
            _logger.Warning(e, "[{Thread}] {Msg}", GetThread(), msg);
        }

        public void Error(string msg, Exception? e)
        {
            _logger.Error(e, "[{Thread}] {Msg}", GetThread(), msg);
        }

        public void Fatal(string msg, Exception? e)
        {
            _logger.Fatal(e, "[{Thread}] {Msg}", GetThread(), msg);
        }

        public void Verbose(string msg, Exception? e)
        {
            _logger.Verbose(e, "[{Thread}] {Msg}", GetThread(), msg);
        }

        /// <summary>
        ///     获取堆栈信息【不要传递任何信息】
        /// </summary>
        /// <param name="file"></param>
        /// <param name="line"></param>
        /// <param name="methodName"></param>
        /// <returns></returns>
        public static string GetStackInfo([CallerFilePath] string file = "", [CallerLineNumber] int line = 0,
            [CallerMemberName] string methodName = "")
        {
            // StackTrace st = new StackTrace(new StackFrame(true));
            // StackFrame sf = st.GetFrame(0);
            // Console.WriteLine(" File: {0}", sf.GetFileName());
            // Console.WriteLine(" Method: {0}", sf.GetMethod().Name);
            // Console.WriteLine(" Line Number: {0}", sf.GetFileLineNumber());
            // Console.WriteLine(" Column Number: {0}", sf.GetFileColumnNumber());

            return $"文件位置:{file},行号:{line},函数名:{methodName}";
        }

        public static void main()
        {
            ILogger log = new SerilogLogger();
        }
    }
}
