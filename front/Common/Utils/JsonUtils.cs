using Newtonsoft.Json;

namespace Common.Utils
{
    public class JsonUtils
    {
        /// <summary>
        ///     序列化
        /// </summary>
        /// <param name="obj"></param>
        /// <returns></returns>
        public static string Serialize(object obj)
        {
            return JsonConvert.SerializeObject(obj);
        }

        /// <summary>
        ///     翻序列化
        /// </summary>
        /// <param name="str">要反序列化的字符串</param>
        /// <typeparam name="T">要序列化的格式</typeparam>
        /// <returns></returns>
        public static T? Deserialize<T>(string str)
        {
            return JsonConvert.DeserializeObject<T>(str);
        }
    }
}
