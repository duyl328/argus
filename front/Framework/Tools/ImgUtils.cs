using System.Windows.Media.Imaging;

namespace Framework.Tools;

public static class ImgUtils
{
    public static (int width, int height) GetImageDimensions(string imagePath)
    {
        try
        {
            var bitmap = new BitmapImage(new Uri(imagePath));
            return (bitmap.PixelWidth, bitmap.PixelHeight);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error loading image: {ex.Message}");
            return (0, 0); // 返回宽高为0，表示加载失败
        }
    }

}
