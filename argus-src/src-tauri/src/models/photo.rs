use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

/// 图片信息
#[derive(Queryable, Selectable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::storage::schema::photo_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Photo {
    pub id: i32,
    /// 图像路径
    pub img_path: String,
    /// 文件名称
    pub img_name: String,
    /// 文件 Hash【唯一 ID】
    pub hash: String,
    /// 图片宽度。
    pub width: i32,
    /// 图片高度
    pub height: i32,
    /// 图片比例（宽/高，方便快速排序）。
    pub aspect_ratio: f32,
    /// 文件大小（字节）。
    pub file_size: i64,
    /// 图片格式（如 JPEG, PNG, WebP）。
    pub format: String,

    /// 备注信息。
    pub notes: Option<String>,

    /*
        相册存储：
            对于相册存储来说，相册 和 图片 都是单独的文件夹，然后通过管理表，一列存储相册ID一列存储图片ID将二者进行互联
        标签：
            【标签表的内容：用户设置、软件生成、是否成为单独标签（特殊标签比如收藏、账单、隐藏）】
            预删除、未审阅、

            和相册类似方式
            收藏、隐藏 等功能都通过标签实现

        人脸：
            人脸进行单独存储，使用单独的表进行人脸和图片的链接

        地点：
            读取所有gps信息不为空的图片，并解析计算，按照分簇展示


        未分类照片过滤：
            是否已标注（标注删除或保存）

    diesel migration generate photo_table_add_info_field

        图片查看次数

        */
    
    
    /// 是否经过算法【识别 人物、场景 等内容】
    pub is_algorithm: Option<bool>,

    /// 算法评分
    pub algorithm_score: Option<i32>,

    /// 上次查看时间
    pub last_viewed_time: Option<i64>,

    // region exif 信息
    /// 时区 存储时区偏移量，如 '+08:00' 或 '-05:00'
    pub offset_time: Option<String>,
    /// 评分（用户评分）
    pub rating: Option<i32>,
    // 评分（自动评分）
    /// 相机制造商
    pub make: Option<String>,
    /// 相机型号
    pub model: Option<String>,
    /// 软件版本
    pub software: Option<String>,
    /// 曝光时间
    pub exposure_time: Option<f32>,
    /// 闪光灯
    pub flash: Option<String>,
    /// 光圈
    pub f_number: Option<f32>,
    /// ISO
    pub iso: Option<i32>,
    /// 创建日期
    pub date_time_original: Option<i64>,
    /// 最大光圈值
    pub max_aperture_value: Option<String>,
    /// 焦距
    pub focal_length: Option<f32>,
    /// 宽度
    pub image_width: Option<i32>,
    /// 长度
    pub image_height: Option<i32>,
    /// gps 信息
    pub gps_info: Option<String>,
    /// 曝光程序
    pub exposure_program: Option<String>,
    /// 测光模式
    pub metering_mode: Option<String>,
    /// 作者（艺术家）
    pub artist: Option<String>,
    // endregion
    pub is_delete: bool,
    pub create_time: i64,
    pub update_time: i64,
}

#[derive(Insertable)]
#[diesel(table_name = crate::storage::schema::photo_table)]
pub struct NewPhoto {
    /// 图像路径
    pub img_path: String,
    /// 文件名称
    pub img_name: String,
    /// 文件 Hash【唯一 ID】
    pub hash: String,
    /// 图片宽度。
    pub width: i32,
    /// 图片高度
    pub height: i32,
    /// 图片比例（宽/高，方便快速排序）。
    pub aspect_ratio: f32,
    /// 文件大小（字节）。
    pub file_size: i64,
    /// 图片格式（如 JPEG, PNG, WebP）。
    pub format: String,
    pub create_time: i64,
    pub update_time: i64,

}

/*

id：图片唯一标识符（主键，自增）。
file_path：图片文件的路径。
thumbnail_path：缩略图路径。
width：图片宽度。
height：图片高度。
aspect_ratio：图片比例（宽/高，方便快速排序）。
file_size：文件大小（字节）。
format：图片格式（如 JPEG, PNG, WebP）。
created_at：图片文件创建时间。
added_at：图片加入数据库的时间。
notes：备注信息。
文件名
Hash

ISO
曝光时间
光圈
焦距
相机型号
拍摄时间
拍摄地点
拍摄者
拍摄场景
关键字
版权
质量得分
经度
维度
海拔
国家
时区


按照同一个字段的标签进行划分
nsfw 检测
人物检测
人脸检测、识别
物体检测
场景检测
文字检测、识别
以图搜图



*/
