namespace Repository.entity
{
    /// <summary>
    ///     相册存储路径
    /// </summary>
    public class PhotoStoragePath : BaseModel
    {
        /// <summary>
        ///     照片存储路径【路径绝对唯一】
        /// </summary>
        public string StoragePath { get; set; } = "";

        /// <summary>
        ///     是否启用
        /// </summary>
        public bool IsEnable { get; set; }

        public PhotoStoragePath(string storagePath, bool isEnable)
        {
            StoragePath = storagePath;
            IsEnable = isEnable;
        }

        public PhotoStoragePath()
        {
        }

        public override string ToString()
        {
            return $"StoragePath:{StoragePath}, IsEnable:${IsEnable}";
        }
    }
}
