use crate::constant::DEFAULT_THUMBNAIL_SIZE;

/// 对图像大小的描述
pub struct ImageSize {
    /// 描述
    // pub description:String,

    /// 图像大小
    pub size: u32,
}

impl ImageSize {
    pub fn default() -> ImageSize {
        ImageSize { size: DEFAULT_THUMBNAIL_SIZE }
    }
}
