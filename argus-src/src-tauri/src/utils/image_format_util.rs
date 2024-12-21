use image::ImageFormat;

/// 通过图片格式获取匹配文件名
pub fn get_suffix_name(image_format: ImageFormat) -> String {
   match image_format {
        ImageFormat::Png => {String::from("png")}
        ImageFormat::Jpeg => {String::from("jpg")}
        ImageFormat::Gif => {String::from("gif")}
        ImageFormat::WebP => {String::from("webp")}
        ImageFormat::Pnm => {String::from("pnm")}
        ImageFormat::Tiff => {String::from("tiff")}
        ImageFormat::Tga => {String::from("tga")}
        ImageFormat::Dds => {String::from("dds")}
        ImageFormat::Bmp => {String::from("bmp")}
        ImageFormat::Ico => {String::from("ico")}
        ImageFormat::Hdr => {String::from("hdr")}
        ImageFormat::OpenExr => {String::from("exr")}
        ImageFormat::Farbfeld => {String::from("ff")}
        ImageFormat::Avif => {String::from("avif")}
        ImageFormat::Qoi => {String::from("qoi")}
        ImageFormat::Pcx => {String::from("pcx")}
       _ => {
           panic!("未知的图片类型! ");
       }
   }
}
