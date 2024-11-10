namespace Common.Utils;

public static class FileTypeChecker
{
    // 图片和视频的常见扩展名
    private static readonly string[] ImageExtensions = { ".jpg", ".jpeg", ".png", ".bmp", ".gif", ".tiff", ".svg", ".webp" };
    private static readonly string[] VideoExtensions = { ".mp4", ".mov", ".avi", ".mkv", ".flv", ".wmv", ".webm", ".3gp" };

    public static bool IsImageFile(string filePath)
    {
        string? extension = Path.GetExtension(filePath)?.ToLower();
        if (extension == null)
        {
            return false;
        }
        return Array.Exists(ImageExtensions, ext => ext == extension);
    }

    public static bool IsVideoFile(string filePath)
    {
        string? extension = Path.GetExtension(filePath)?.ToLower();
        if (extension == null)
        {
            return false;
        }
        return Array.Exists(VideoExtensions, ext => ext == extension);
    }
}
