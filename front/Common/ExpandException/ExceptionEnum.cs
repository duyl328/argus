namespace Common.ExpandException
{
    public class ExceptionEnum
    {
        // region 数据库相关错误 错误码: 10XXXX

        /// <summary>
        ///     照片存储路径不能为空
        /// </summary>
        public static readonly ExpandException PhotoStoragePathCannotEmpty = new(100001L, "照片存储路径不能为空!", "检查输入路径!");
        /// <summary>
        ///     不能重复添加路径
        /// </summary>
        public static readonly ExpandException PathsCannotRepeatedly = new(100002L, "不能重复添加路径!", "检查输入路径!");
        /// <summary>
        ///     照片存储路径不能为 Null
        /// </summary>
        public static readonly ExpandException PhotoStoragePathCannotIsNull = new(100003L, "照片存储路径不能为 Null!", "检查输入路径是否为空!");
        /// <summary>
        ///     路径未找到的异常
        /// </summary>
        public static readonly ExpandException PhotoStoragePathNotFound = new(100004L, "路径未找到的异常!", "检查输入路径!");


        // endregion
    }
}
