/// 数值类型【目前仅处理 gps ，默认均为字符串】
#[derive(Clone, Debug)]
pub enum ValueType {
    String,
    Gps,
    // 时间处理暂未使用
    Time,
}
pub trait ExifValueConverter {
    fn convert(value: &str) -> Self;
}

impl ExifValueConverter for ValueType {
    fn convert(value: &str)->Self  {
        value.to_string();
        return ValueType::String;
    }
}
// impl ExifValueConverter for ValueType::Gps {
//     fn convert(value: &str) -> Self {
//         value.to_string()
//     }
// }
// impl ExifValueConverter for ValueType::Time {
//     fn convert(value: &str) -> Self {
//         value.to_string()
//     }
// }

// 1. Byte(Vec<u8>)
// 用途：表示一组 8 位无符号整数（字节）。通常用于存储像 JPEG 图像文件中的二进制数据。
// 为什么需要：EXIF 数据中可能包含一些二进制文件，比如缩略图，或者用于存储不符合其他类型格式的数据。
// 2. Ascii(Vec<Vec<u8>>)
// 用途：表示一个或多个 ASCII 字符串，每个字符串为一个字节的向量。
// 为什么需要：一些 EXIF 字段（如相机品牌、型号）使用 ASCII 编码，通常是由一组 ASCII 字符组成。
// 3. Short(Vec<u16>)
// 用途：表示 16 位无符号整数的向量。
// 为什么需要：EXIF 中的一些字段（如曝光时间）使用 16 位无符号整数表示。
// 4. Long(Vec<u32>)
// 用途：表示 32 位无符号整数的向量。
// 为什么需要：EXIF 中的某些字段需要使用 32 位无符号整数来表示数据。
// 5. Rational(Vec<Rational>)
// 用途：表示有理数（即两个整数的比值）的向量。
// 为什么需要：EXIF 中的某些数据字段（如曝光时间、焦距等）可能会用有理数来表示。Rational 类型是为了处理这种情况的。
// 6. SByte(Vec<i8>)
// 用途：表示 8 位有符号整数的向量，通常在 EXIF 中不常用。
// 为什么需要：虽然 EXIF 标准中很少使用 SByte，但这个类型仍然被保留，因为某些扩展可能会使用它。
// 7. Undefined(Vec<u8>, u32)
// 用途：表示不明确的数据，它存储的是原始字节流和相关的偏移量信息。
// 为什么需要：一些 EXIF 数据可能不遵循明确的格式，Undefined 类型提供了一种通用方式来处理这类数据。
// 8. SShort(Vec<i16>)
// 用途：表示 16 位有符号整数的向量。EXIF 规范中未指定使用，但这个类型可以处理一些特定情况。
// 为什么需要：尽管未在 EXIF 规范中广泛使用，但为了扩展性和通用性，仍然定义了 SShort 类型。
// 9. SLong(Vec<i32>)
// 用途：表示 32 位有符号整数的向量。
// 为什么需要：EXIF 中的一些字段可能需要有符号整数表示，例如某些表示偏移量或计数的字段。
// 10. SRational(Vec<SRational>)
// 用途：表示带符号的有理数向量。SRational 是由两个带符号的整数组成的。
// 为什么需要：EXIF 数据中可能包含带符号的有理数值，比如某些计算结果可能使用带符号的数值。
// 11. Float(Vec<f32>)
// 用途：表示 32 位单精度浮点数的向量。
// 为什么需要：一些 EXIF 字段可能包含浮点数数据，虽然 EXIF 标准中并不广泛使用浮点类型，但它依然被保留以支持其他使用场景。
// 12. Double(Vec<f64>)
// 用途：表示 64 位双精度浮点数的向量。
// 为什么需要：与 Float 类型类似，这个类型用于处理精度要求更高的浮点数数据。
// 13. Unknown(u16, u32, u32)
// 用途：表示未知类型的数据，这包括未知类型的值、计数和偏移量。
// 为什么需要：如果 EXIF 数据中遇到不符合标准的数据格式，或者是某些扩展字段，Unknown 类型提供了一个通用的方式来存储这些数据。
