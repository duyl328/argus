use crate::tuples::Pair;
use crate::utils::exif_utils::gps_util::GpsInfo;
use crate::utils::exif_utils::value::ValueType;
use crate::utils::json_util::JsonUtil;
use anyhow::{anyhow, Result};
use chrono::{DateTime, FixedOffset, Utc};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Tags {
    /// 原始数据保存
    pub entries: Vec<(String, String)>,
    /// 数据映射
    pub entry_map: HashMap<String, String>,
    /// 遇到错误时继续
    continue_on_error: bool,
}

impl Tags {
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

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() || self.entry_map.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.entry_map
            .get(key)
            .map(|v| Cow::Borrowed(v).to_string())
    }

    /// 打包数据
    pub fn pack_tags(&self) -> Result<String> {
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

    /// 打包为前端展示字段
    pub fn pack_front_tags(&self) -> Result<String> {
        let mut res: Vec<Pair<String, String>> = Vec::new();
        let img_exif = self.pack_object()?;

        // 使用一个辅助函数处理字段的封装
        let mut add_tag = |desc: String, value: String| {
            res.push(Pair {
                first: desc,
                second: value,
            });
        };

        if let Some(x) = img_exif.make {
            add_tag(ExifToolDesc::MAKE.dis.to_string(), x);
        }

        if let Some(x) = img_exif.software {
            add_tag(ExifToolDesc::SOFTWARE.dis.to_string(), x);
        }

        if let Some(x) = img_exif.exposure_time {
            add_tag(ExifToolDesc::EXPOSURE_TIME.dis.to_string(), x.to_string());
        }

        if let Some(x) = img_exif.flash {
            add_tag(ExifToolDesc::FLASH.dis.to_string(), x);
        }

        if let Some(x) = img_exif.f_number {
            add_tag(ExifToolDesc::F_NUMBER.dis.to_string(), x.to_string());
        }

        if let Some(x) = img_exif.iso {
            add_tag(ExifToolDesc::ISO.dis.to_string(), x.to_string());
        }

        if let Some(x) = img_exif.date_time_original {
            add_tag(
                ExifToolDesc::DATE_TIME_ORIGINAL.dis.to_string(),
                x.to_string(),
            );
        }

        if let Some(x) = img_exif.max_aperture_value {
            add_tag(ExifToolDesc::MAX_APERTURE_VALUE.dis.to_string(), x);
        }

        if let Some(x) = img_exif.focal_length {
            add_tag(ExifToolDesc::FOCAL_LENGTH.dis.to_string(), x.to_string());
        }

        if let Some(x) = img_exif.image_width {
            add_tag(ExifToolDesc::IMAGE_WIDTH.dis.to_string(), x.to_string());
        }

        if let Some(x) = img_exif.image_height {
            add_tag(ExifToolDesc::IMAGE_HEIGHT.dis.to_string(), x.to_string());
        }

        if let Some(x) = img_exif.gps_info {
            add_tag(String::from("GPS 信息"), x.to_string());
        }

        if let Some(x) = img_exif.exposure_program {
            add_tag(ExifToolDesc::EXPOSURE_PROGRAM.dis.to_string(), x);
        }

        if let Some(x) = img_exif.metering_mode {
            add_tag(ExifToolDesc::METERING_MODE.dis.to_string(), x);
        }

        if let Some(x) = img_exif.artist {
            add_tag(ExifToolDesc::ARTIST.dis.to_string(), x);
        }

        JsonUtil::stringify(&res)
    }

    /// 打包为对象
    pub fn pack_object(&self) -> Result<ImgExif> {
        let make: Option<String>;
        let model: Option<String>;
        let software: Option<String>;
        let exposure_time: Option<f64>;
        let flash: Option<String>;
        let f_number: Option<f64>;
        let iso: Option<u32>;
        let date_time_original: Option<DateTime<Utc>>;
        let max_aperture_value: Option<String>;
        let focal_length: Option<f64>;
        let image_width: Option<u32>;
        let image_height: Option<u32>;
        let gps_info: Option<GpsInfo>;
        let exposure_program: Option<String>;
        let metering_mode: Option<String>;
        let artist: Option<String>;
        let rating: Option<u32>;

        make = self.get(ExifToolDesc::MAKE.exif_tool_desc);
        model = self.get(ExifToolDesc::MODEL.exif_tool_desc);
        software = self.get(ExifToolDesc::SOFTWARE.exif_tool_desc);
        artist = self.get(ExifToolDesc::ARTIST.exif_tool_desc);
        flash = self.get(ExifToolDesc::FLASH.exif_tool_desc);
        focal_length = self.parse_number_data(ExifToolDesc::FOCAL_LENGTH.exif_tool_desc)?;
        // 解析曝光时间信息
        exposure_time = self.parse_exposure_time();
        // 光圈数解析
        f_number = self.parse_number_data(ExifToolDesc::F_NUMBER.exif_tool_desc)?;
        image_width = self.parse_number_data(ExifToolDesc::IMAGE_WIDTH.exif_tool_desc)?;
        image_height = self.parse_number_data(ExifToolDesc::IMAGE_HEIGHT.exif_tool_desc)?;
        max_aperture_value =
            self.parse_number_data(ExifToolDesc::MAX_APERTURE_VALUE.exif_tool_desc)?;
        iso = self.parse_number_data(ExifToolDesc::ISO.exif_tool_desc)?;
        exposure_program = self.get(ExifToolDesc::EXPOSURE_PROGRAM.exif_tool_desc);
        metering_mode = self.get(ExifToolDesc::METERING_MODE.exif_tool_desc);
        // 解析 GPS
        gps_info = Option::from(GpsInfo::parse(self, self.continue_on_error)?);
        // 解析时间
        date_time_original = self.parse_create_time();
        // 评分
        rating = self.parse_number_data(ExifToolDesc::RATING.exif_tool_desc)?;
        Ok(ImgExif {
            make,
            model,
            software,
            exposure_time,
            flash,
            f_number,
            iso,
            date_time_original,
            max_aperture_value,
            focal_length,
            image_width,
            image_height,
            gps_info,
            exposure_program,
            metering_mode,
            artist,
            rating
        })
    }

    /// 解析时间
    pub fn parse_create_time(&self) -> Option<DateTime<Utc>> {
        let create_time: Option<String> = self.get(ExifToolDesc::DATE_TIME_ORIGINAL.exif_tool_desc);
        let offset_time: Option<String> = self.get(ExifToolDesc::OFFSET_TIME.exif_tool_desc);

        // 如果 create_time 是 None，直接返回 None
        let date_str = create_time?;

        // 如果 offset_time 是 None，则使用默认的东八区时区 "+08:00"
        let offset_str = offset_time.unwrap_or_else(|| "+08:00".to_string());

        // 解析 Date/Time Original 字符串为 DateTime<FixedOffset>
        let date_time = DateTime::parse_from_str(&date_str, "%Y:%m:%d %H:%M:%S").ok()?;

        // 解析 Offset Time 字符串为 FixedOffset
        let offset = FixedOffset::from_str(&offset_str).ok()?;

        // 使用时区偏移创建 DateTime<FixedOffset>，然后转换为 UTC 时间
        Some(date_time.with_timezone(&offset).with_timezone(&Utc))
    }

    pub fn parse_number_data<T>(&self, str: &str) -> Result<Option<T>>
    where
        T: FromStr + Default,
    {
        self.get(str).map_or(Ok(None), |x| match x.parse::<T>() {
            Ok(value) => Ok(Some(value)),
            Err(_) if self.continue_on_error => Ok(Some(T::default())),
            Err(_e) => Err(anyhow!(format!("数据: {} 转换失败! ", x))),
        })
    }
    /// 曝光时间
    pub fn parse_exposure_time(&self) -> Option<f64> {
        let option = self.get(ExifToolDesc::EXPOSURE_TIME.exif_tool_desc);
        if let Some(x) = option {
            let parts: Vec<&str> = x.split('/').collect();
            if parts.len() == 2 {
                let numerator: f64 = parts[0].parse().unwrap_or(1.0);
                let denominator: f64 = parts[1].parse().unwrap_or(1.0);
                Some(numerator / denominator)
            } else {
                // 如果格式不对
                None
            }
        } else {
            None
        }
    }

    /// 焦距解析
    pub fn parse_focal_length(&self) -> Option<String> {
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

            Some(ans)
        } else {
            None
        }
    }

    /// 解析 gps 数据【获取 gps 数据，并根据有无转换为文字信息】
    pub fn parse_gps_tags(&self) -> Result<GpsInfo> {
        Ok(GpsInfo::parse(self, true)?)
    }

    pub fn new(continue_on_error: bool) -> Self {
        Tags {
            entries: Vec::new(),
            entry_map: HashMap::new(),
            continue_on_error,
        }
    }
}

/// 图像的 exif 信息对象
#[derive(Debug, Clone)]
pub struct ImgExif {
    /// 相机制造商
    pub make: Option<String>,
    /// 相机型号
    pub model: Option<String>,
    /// 软件版本
    pub software: Option<String>,
    /// 曝光时间
    pub exposure_time: Option<f64>,
    /// 闪光灯
    pub flash: Option<String>,
    /// 光圈
    pub f_number: Option<f64>,
    /// ISO
    pub iso: Option<u32>,
    /// exif 信息版本
    // exif_version:OptionString>,
    /// 创建日期
    pub date_time_original: Option<DateTime<Utc>>,
    /// 最大光圈值
    pub max_aperture_value: Option<String>,
    /// 焦距
    pub focal_length: Option<f64>,
    /// 宽度
    pub image_width: Option<u32>,
    /// 长度
    pub image_height: Option<u32>,
    /// gps 信息
    pub gps_info: Option<GpsInfo>,
    /// 曝光程序
    pub exposure_program: Option<String>,
    /// 测光模式
    pub metering_mode: Option<String>,
    /// 作者（艺术家）
    pub artist: Option<String>,
    /// 等级【评分】
    pub rating: Option<u32>,
}

impl fmt::Display for ImgExif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ans_str = String::new();
        if let Some(x) = &self.make {
            ans_str.push_str(x.as_str());
        }

        if let Some(x) = &self.model {
            ans_str.push_str(x.as_str());
        }

        if let Some(x) = &self.software {
            ans_str.push_str(x.as_str());
        }

        if let Some(x) = &self.exposure_time {
            ans_str.push_str(x.to_string().as_str());
        }

        if let Some(x) = &self.flash {
            ans_str.push_str(x.as_str());
        }

        if let Some(x) = &self.f_number {
            ans_str.push_str(x.to_string().as_str());
        }

        if let Some(x) = &self.iso {
            ans_str.push_str(x.to_string().as_str());
        }

        if let Some(x) = &self.date_time_original {
            ans_str.push_str(x.to_string().as_str());
        }

        if let Some(x) = &self.max_aperture_value {
            ans_str.push_str(x.to_string().as_str());
        }

        if let Some(x) = &self.focal_length {
            ans_str.push_str(x.to_string().as_str());
        }

        if let Some(x) = &self.image_width {
            ans_str.push_str(x.to_string().as_str());
        }

        if let Some(x) = &self.image_height {
            ans_str.push_str(x.to_string().as_str());
        }

        if let Some(x) = &self.gps_info {
            ans_str.push_str(x.to_string().as_str());
        }

        if let Some(x) = &self.exposure_program {
            ans_str.push_str(x.to_string().as_str());
        }

        if let Some(x) = &self.metering_mode {
            ans_str.push_str(x.to_string().as_str());
        }

        if let Some(x) = &self.artist {
            ans_str.push_str(x.to_string().as_str());
        }
        if let Some(x) = &self.rating {
            ans_str.push_str(x.to_string().as_str());
        }
        write!(f, "{}", ans_str)
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
    pub const RATING: ExifInfo = ExifInfo {
        dis: "评级",
        exif_tool_desc: "Rating",
        value_type: ValueType::String,
    };

    pub const EXIF_INFOS: [&'static ExifInfo; 24] = [
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
        &Self::RATING,
    ];
    /// 前端展示的数据
    pub const EXIF_INFOS_FRONT: [&'static ExifInfo; 24] = ExifToolDesc::EXIF_INFOS;
}

#[derive(Clone, Debug)]
pub struct ExifInfo {
    pub dis: &'static str,
    /// exiftool 的文字描述（匹配数据）
    pub exif_tool_desc: &'static str,
    /// 数据类型
    pub value_type: ValueType,
}

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
