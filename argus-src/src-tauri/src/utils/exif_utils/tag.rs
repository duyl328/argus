use crate::tuples::Pair;
use crate::utils::exif_utils::value::ValueType;
use crate::utils::json_util::JsonUtil;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::format;

#[derive(Debug, Clone)]
pub struct Tag {
    /// 原始数据保存
    pub entries: Vec<(String, String)>,
    /// 数据映射
    pub entry_map: HashMap<String, String>,
}

impl Tag {
    pub fn parse(mut self, info: &str) -> Self {
        for line in info.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();
                self.entry_map.insert(key.clone(), value.clone());
                self.entries.push((key, value));
            }
        }
        self
    }

    pub fn get(&self, key: &str) -> Option<Cow<String>> {
        self.entry_map.get(key).map(|v| Cow::Borrowed(v))
    }
// todo: 2025/1/4 22:29 GPS 信息的单独处理 （根据维度参考，具体维度信息，以及高度进行处理）
    /// 打包数据
    pub fn pack_tags(&self) -> anyhow::Result<String> {
        let mut res: Vec<Pair<String, String>> = Vec::new();
        ExifToolDesc::EXIF_INFOS_FRONT.map(|info| {
            let ans = self.get(info.exif_tool_desc);
            // 如果数据有值
            if ans.is_some() {
                res.push(Pair {
                    first: info.dis.to_string(),
                    second: ans.unwrap().to_string(),
                });
            }
        });
        JsonUtil::stringify(&res)
    }
    pub fn pack_front_tags(&self) -> anyhow::Result<String> {
        let mut res: Vec<Pair<String, String>> = Vec::new();

        // 使用一个辅助函数处理字段的封装
        let mut add_tag = |desc: &ExifInfo, field_name: &str| {
            self.get(desc.exif_tool_desc).map(|x| {
                res.push(Pair {
                    first: field_name.to_string(),
                    second: x.to_string(),
                });
            });
        };

        // 封装通用的字段添加逻辑
        add_tag(&ExifToolDesc::MAKE, ExifToolDesc::MAKE.dis);
        add_tag(&ExifToolDesc::MODEL, ExifToolDesc::MODEL.dis);
        add_tag(&ExifToolDesc::SOFTWARE, ExifToolDesc::SOFTWARE.dis);
        add_tag(&ExifToolDesc::ARTIST, ExifToolDesc::ARTIST.dis);
        add_tag(&ExifToolDesc::FLASH, ExifToolDesc::FLASH.dis);
        add_tag(&ExifToolDesc::FOCAL_LENGTH, ExifToolDesc::FOCAL_LENGTH.dis);
        add_tag(
            &ExifToolDesc::EXPOSURE_TIME,
            ExifToolDesc::EXPOSURE_TIME.dis,
        );
        add_tag(&ExifToolDesc::F_NUMBER, ExifToolDesc::F_NUMBER.dis);
        add_tag(&ExifToolDesc::ISO, ExifToolDesc::ISO.dis);
        add_tag(
            &ExifToolDesc::EXPOSURE_PROGRAM,
            ExifToolDesc::EXPOSURE_PROGRAM.dis,
        );
        add_tag(
            &ExifToolDesc::METERING_MODE,
            ExifToolDesc::METERING_MODE.dis,
        );

        // 焦距和等效焦距处理
        if let Some(focal_length) = self.get(ExifToolDesc::FOCAL_LENGTH.exif_tool_desc) {
            let mm35 = self
                .get(ExifToolDesc::FOCAL_LENGTH_IN_35MM_FORMAT.exif_tool_desc)
                .unwrap_or_default();
            let ans = if mm35.is_empty() {
                focal_length.to_string()
            } else {
                format!("{}, 等效焦距: {}", focal_length, mm35)
            };

            res.push(Pair {
                first: ExifToolDesc::FOCAL_LENGTH.dis.to_string(),
                second: ans,
            });
        }

        // GPS 信息拼接
        let gps_ans = [
            self.get(ExifToolDesc::GPS_LATITUDE_REF.exif_tool_desc),
            self.get(ExifToolDesc::GPS_LATITUDE.exif_tool_desc),
            self.get(ExifToolDesc::GPS_LONGITUDE_REF.exif_tool_desc),
            self.get(ExifToolDesc::GPS_LONGITUDE.exif_tool_desc),
            self.get(ExifToolDesc::GPS_ALTITUDE.exif_tool_desc),
        ]
        .iter()
        .filter_map(|x| x.clone().map(|cow| cow.to_string()))
        .collect::<Vec<String>>()
        .join(" "); // 连接所有字段

        res.push(Pair {
            first: "GPS 信息".to_string(),
            second: gps_ans,
        });

        JsonUtil::stringify(&res)
    }

    pub fn new() -> Self {
        Tag {
            entries: Vec::new(),
            entry_map: HashMap::new(),
        }
    }
}

pub struct ExifToolDesc {}

impl ExifToolDesc {
    pub const MAKE: ExifInfo = ExifInfo {
        dis: "相机制造商",
        exif_tool_desc: "Make",
        value_type: ValueType::String,
    };
    pub const MODEL: ExifInfo = ExifInfo {
        dis: "相机型号",
        exif_tool_desc: "Camera Model Name",
        value_type: ValueType::String,
    };
    pub const SOFTWARE: ExifInfo = ExifInfo {
        dis: "软件",
        exif_tool_desc: "Software",
        value_type: ValueType::String,
    };
    pub const EXPOSURE_TIME: ExifInfo = ExifInfo {
        dis: "快门速度",
        exif_tool_desc: "Exposure Time",
        value_type: ValueType::String,
    };
    pub const F_NUMBER: ExifInfo = ExifInfo {
        dis: "光圈数",
        exif_tool_desc: "F Number",
        value_type: ValueType::String,
    };
    pub const ISO: ExifInfo = ExifInfo {
        dis: "ISO 感光度",
        exif_tool_desc: "ISO",
        value_type: ValueType::String,
    };
    pub const EXIF_VERSION: ExifInfo = ExifInfo {
        dis: "Exif 版本",
        exif_tool_desc: "Exif Version",
        value_type: ValueType::String,
    };
    pub const DATE_TIME_ORIGINAL: ExifInfo = ExifInfo {
        dis: "拍摄时间",
        exif_tool_desc: "Date/Time Original",
        value_type: ValueType::String,
    };
    pub const OFFSET_TIME: ExifInfo = ExifInfo {
        dis: "时区",
        exif_tool_desc: "Offset Time",
        value_type: ValueType::String,
    };
    pub const MAX_APERTURE_VALUE: ExifInfo = ExifInfo {
        dis: "最大光圈",
        exif_tool_desc: "Max Aperture Value",
        value_type: ValueType::String,
    };
    pub const FOCAL_LENGTH: ExifInfo = ExifInfo {
        dis: "焦距",
        exif_tool_desc: "Focal Length",
        value_type: ValueType::String,
    };
    pub const FOCAL_LENGTH_IN_35MM_FORMAT: ExifInfo = ExifInfo {
        dis: "等效焦距",
        exif_tool_desc: "Focal Length In 35mm Format",
        value_type: ValueType::String,
    };
    pub const IMAGE_WIDTH: ExifInfo = ExifInfo {
        dis: "图像宽度",
        exif_tool_desc: "Image Width",
        value_type: ValueType::String,
    };
    pub const IMAGE_HEIGHT: ExifInfo = ExifInfo {
        dis: "图像长度",
        exif_tool_desc: "Image Height",
        value_type: ValueType::String,
    };
    pub const GPS_LATITUDE_REF: ExifInfo = ExifInfo {
        dis: "GPS 纬度参考",
        exif_tool_desc: "GPS Latitude Ref",
        value_type: ValueType::String,
    };
    pub const GPS_LONGITUDE_REF: ExifInfo = ExifInfo {
        dis: "GPS 经度参考",
        exif_tool_desc: "GPS Longitude Ref",
        value_type: ValueType::String,
    };
    pub const GPS_LATITUDE: ExifInfo = ExifInfo {
        dis: "GPS 纬度",
        exif_tool_desc: "GPS Latitude",
        value_type: ValueType::String,
    };
    pub const GPS_LONGITUDE: ExifInfo = ExifInfo {
        dis: "GPS 经度",
        exif_tool_desc: "GPS Longitude",
        value_type: ValueType::String,
    };
    pub const GPS_ALTITUDE: ExifInfo = ExifInfo {
        dis: "GPS 海拔",
        exif_tool_desc: "GPS Altitude",
        value_type: ValueType::String,
    };
    pub const EXPOSURE_PROGRAM: ExifInfo = ExifInfo {
        dis: "曝光程序",
        exif_tool_desc: "Exposure Program",
        value_type: ValueType::String,
    };
    pub const METERING_MODE: ExifInfo = ExifInfo {
        dis: "测光模式",
        exif_tool_desc: "Metering Mode",
        value_type: ValueType::String,
    };
    pub const FLASH: ExifInfo = ExifInfo {
        dis: "闪光灯",
        exif_tool_desc: "Flash",
        value_type: ValueType::String,
    };
    pub const ARTIST: ExifInfo = ExifInfo {
        dis: "艺术家",
        exif_tool_desc: "Artist",
        value_type: ValueType::String,
    };

    pub const EXIF_INFOS: [&'static ExifInfo; 23] = [
        &Self::MAKE,
        &Self::MODEL,
        &Self::SOFTWARE,
        &Self::EXPOSURE_TIME,
        &Self::F_NUMBER,
        &Self::ISO,
        &Self::EXIF_VERSION,
        &Self::DATE_TIME_ORIGINAL,
        &Self::OFFSET_TIME,
        &Self::MAX_APERTURE_VALUE,
        &Self::FOCAL_LENGTH,
        &Self::FOCAL_LENGTH_IN_35MM_FORMAT,
        &Self::IMAGE_WIDTH,
        &Self::IMAGE_HEIGHT,
        &Self::GPS_LATITUDE_REF,
        &Self::GPS_LONGITUDE_REF,
        &Self::GPS_LATITUDE,
        &Self::GPS_LONGITUDE,
        &Self::GPS_ALTITUDE,
        &Self::EXPOSURE_PROGRAM,
        &Self::METERING_MODE,
        &Self::FLASH,
        &Self::ARTIST,
    ];
    /// 前端展示的数据
    pub const EXIF_INFOS_FRONT: [&'static ExifInfo; 23] = [
        &Self::MAKE,
        &Self::MODEL,
        &Self::SOFTWARE,
        &Self::EXPOSURE_TIME,
        &Self::F_NUMBER,
        &Self::ISO,
        &Self::EXIF_VERSION,
        &Self::DATE_TIME_ORIGINAL,
        &Self::OFFSET_TIME,
        &Self::MAX_APERTURE_VALUE,
        &Self::FOCAL_LENGTH,
        &Self::FOCAL_LENGTH_IN_35MM_FORMAT,
        &Self::IMAGE_WIDTH,
        &Self::IMAGE_HEIGHT,
        &Self::GPS_LATITUDE_REF,
        &Self::GPS_LONGITUDE_REF,
        &Self::GPS_LATITUDE,
        &Self::GPS_LONGITUDE,
        &Self::GPS_ALTITUDE,
        &Self::EXPOSURE_PROGRAM,
        &Self::METERING_MODE,
        &Self::FLASH,
        &Self::ARTIST,
    ];
}

#[derive(Clone, Debug)]
pub struct ExifInfo {
    pub dis: &'static str,
    /// exiftool 的文字描述（匹配数据）
    pub exif_tool_desc: &'static str,
    /// 数据类型
    pub value_type: ValueType,
}

// todo: 2025/1/2 21:05 该部分目前只关心 gps ，暂不拓展
/*
现在的目的是实现结构体的定义和现有数据的匹配，数据的解析等功能暂不考虑
*/
// 名称、描述、类型、默认值
// 生成标签常量
// macro_rules! generate_tag_constants {
//     (
//         $(
//             // 捕获传递的属性【可空】
//             // $(#[$attr:meta])*
//             ($name:ident ,$value:expr,$defval:expr, $desc:expr,$ExifToolDesc:expr, $type:ty)
//         )+
//
//     ) => (
//         struct ExifTag {
//             pub const $name: Tag = Tag($ctx, $num);
//         }
//
//     );
// }
//
// generate_tag_constants!();
