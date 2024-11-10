using System;
using System.Xml.Schema;

namespace Common.Utils
{
    /// <summary>
    ///     时间工具类
    /// </summary>
    public class TimeUtils
    {
        /// <summary>
        ///     获取精确时间，精确到毫秒
        /// </summary>
        /// <returns></returns>
        public static string GetPreciseTime()
        {
            return DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss.fff");
        }

        public static void Mains()
        {
            Console.WriteLine($"{DateTime.Now:yyyy-MM-dd}");
        }
    }
}
