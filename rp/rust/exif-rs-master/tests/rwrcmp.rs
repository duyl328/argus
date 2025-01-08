
//! Read, write, re-read, and compare the results.
//!
//! This test can be also compiled as a command-line program.

extern crate exif;

use std::env;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::Path;

use exif::experimental::Writer;
#[cfg(not(test))]
use exif::Error;
use exif::{Exif, In, Reader, Tag, Value};

#[test]
fn exif_heic() {
    rwr_compare("tests/exif.heic");
}

#[test]
fn exif_jpg() {
    rwr_compare("tests/exif.jpg");
}

#[test]
fn exif_png() {
    rwr_compare("tests/exif.png");
}

#[test]
fn exif_tif() {
    rwr_compare("tests/exif.tif");
}

#[test]
fn exif_webp() {
    rwr_compare("tests/exif.webp");
}

fn main() {
    for path in env::args_os().skip(1) {
        rwr_compare(&path);
    }
}

fn rwr_compare<P>(path: P)
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    // Read.
    let file = File::open(path).unwrap();
    let mut bufreader = BufReader::new(&file);
    #[cfg(test)]
    let exif1 = Reader::new().read_from_container(&mut bufreader).unwrap();
    #[cfg(not(test))]
    let exif1 = match Reader::new().read_from_container(&mut bufreader) {
        Ok(exif) => exif,
        Err(e) => {
            println!("{}: {}: Skipped", path.display(), e);
            return;
        }
    };
    let strips = get_strips(&exif1, In::PRIMARY);
    let tn_strips = get_strips(&exif1, In::THUMBNAIL);
    let tiles = get_tiles(&exif1, In::PRIMARY);
    let tn_jpeg = get_jpeg(&exif1, In::THUMBNAIL);

    // Write.
    let mut writer = Writer::new();
    for f in exif1.fields() {
        writer.push_field(f);
    }
    if let Some(ref strips) = strips {
        writer.set_strips(strips, In::PRIMARY);
    }
    if let Some(ref tn_strips) = tn_strips {
        writer.set_strips(tn_strips, In::THUMBNAIL);
    }
    if let Some(ref tiles) = tiles {
        writer.set_tiles(tiles, In::PRIMARY);
    }
    if let Some(ref tn_jpeg) = tn_jpeg {
        writer.set_jpeg(tn_jpeg, In::THUMBNAIL);
    }
    let mut out = Cursor::new(Vec::new());
    #[cfg(test)]
    writer.write(&mut out, exif1.little_endian()).unwrap();
    #[cfg(not(test))]
    match writer.write(&mut out, exif1.little_endian()) {
        Ok(_) => {}
        Err(Error::NotSupported(_)) => {
            println!("{}: Contains unknown type", path.display());
            return;
        }
        e => e.unwrap(),
    }
    let out = out.into_inner();

    // Re-read.
    let exif2 = Reader::new().read_raw(out).unwrap();

    // 对字段进行排序（某些文件的标签顺序错误）。
    let mut fields1 = exif1.fields().collect::<Vec<_>>();
    fields1.sort_by_key(|f| (f.ifd_num, f.tag));
    let mut fields2 = exif2.fields().collect::<Vec<_>>();
    fields2.sort_by_key(|f| (f.ifd_num, f.tag));

    // Compare.
    assert_eq!(fields1.len(), fields2.len());
    for (f1, f2) in fields1.iter().zip(fields2.iter()) {
        assert_eq!(f1.tag, f2.tag);
        assert_eq!(f1.ifd_num, f2.ifd_num);
        match f1.tag {
            Tag::StripOffsets | Tag::TileOffsets | Tag::JPEGInterchangeFormat => continue,
            _ => {}
        }
        compare_field_value(&f1.value, &f2.value);
    }
    assert_eq!(get_strips(&exif2, In::PRIMARY), strips);
    assert_eq!(get_strips(&exif2, In::THUMBNAIL), tn_strips);
    assert_eq!(get_tiles(&exif2, In::PRIMARY), tiles);
    assert_eq!(get_jpeg(&exif2, In::THUMBNAIL), tn_jpeg);
}

// 比较字段值.
fn compare_field_value(value1: &Value, value2: &Value) {
    // The TIFF standard requires that BYTE, SHORT, or LONG values should
    // be accepted for any unsigned integer field.
    if let (Some(it1), Some(it2)) = (value1.iter_uint(), value2.iter_uint()) {
        assert!(it1.eq(it2));
        return;
    }
    // Compare other fields strictly.
    match (value1, value2) {
        (&Value::Ascii(ref v1), &Value::Ascii(ref v2)) => assert_eq!(v1, v2),
        (&Value::Rational(ref v1), &Value::Rational(ref v2)) => {
            assert_eq!(v1.len(), v2.len());
            for (r1, r2) in v1.iter().zip(v2.iter()) {
                assert_eq!(r1.num, r2.num);
                assert_eq!(r1.denom, r2.denom);
            }
        }
        (&Value::SByte(ref v1), &Value::SByte(ref v2)) => assert_eq!(v1, v2),
        (&Value::Undefined(ref v1, _), &Value::Undefined(ref v2, _)) => assert_eq!(v1, v2),
        (&Value::SShort(ref v1), &Value::SShort(ref v2)) => assert_eq!(v1, v2),
        (&Value::SLong(ref v1), &Value::SLong(ref v2)) => assert_eq!(v1, v2),
        (&Value::SRational(ref v1), &Value::SRational(ref v2)) => {
            assert_eq!(v1.len(), v2.len());
            for (r1, r2) in v1.iter().zip(v2.iter()) {
                assert_eq!(r1.num, r2.num);
                assert_eq!(r1.denom, r2.denom);
            }
        }
        (&Value::Float(ref v1), &Value::Float(ref v2)) => assert_eq!(v1, v2),
        (&Value::Double(ref v1), &Value::Double(ref v2)) => assert_eq!(v1, v2),
        _ => panic!("{:?} != {:?}", value1, value2),
    }
}

fn get_strips(exif: &Exif, ifd_num: In) -> Option<Vec<&[u8]>> {
    let offsets = exif
        .get_field(Tag::StripOffsets, ifd_num)
        .and_then(|f| f.value.iter_uint());
    let counts = exif
        .get_field(Tag::StripByteCounts, ifd_num)
        .and_then(|f| f.value.iter_uint());
    let (offsets, counts) = match (offsets, counts) {
        (Some(offsets), Some(counts)) => (offsets, counts),
        (None, None) => return None,
        _ => panic!("inconsistent strip offsets and byte counts"),
    };
    let buf = exif.buf();
    assert_eq!(offsets.len(), counts.len());
    let strips = offsets
        .zip(counts)
        .map(|(ofs, cnt)| &buf[ofs as usize..(ofs + cnt) as usize])
        .collect();
    Some(strips)
}

fn get_tiles(exif: &Exif, ifd_num: In) -> Option<Vec<&[u8]>> {
    let offsets = exif
        .get_field(Tag::TileOffsets, ifd_num)
        .and_then(|f| f.value.iter_uint());
    let counts = exif
        .get_field(Tag::TileByteCounts, ifd_num)
        .and_then(|f| f.value.iter_uint());
    let (offsets, counts) = match (offsets, counts) {
        (Some(offsets), Some(counts)) => (offsets, counts),
        (None, None) => return None,
        _ => panic!("inconsistent tile offsets and byte counts"),
    };
    assert_eq!(offsets.len(), counts.len());
    let buf = exif.buf();
    let strips = offsets
        .zip(counts)
        .map(|(ofs, cnt)| &buf[ofs as usize..(ofs + cnt) as usize])
        .collect();
    Some(strips)
}

fn get_jpeg(exif: &Exif, ifd_num: In) -> Option<&[u8]> {
    let offset = exif
        .get_field(Tag::JPEGInterchangeFormat, ifd_num)
        .and_then(|f| f.value.get_uint(0));
    let len = exif
        .get_field(Tag::JPEGInterchangeFormatLength, ifd_num)
        .and_then(|f| f.value.get_uint(0));
    let (offset, len) = match (offset, len) {
        (Some(offset), Some(len)) => (offset as usize, len as usize),
        (None, None) => return None,
        _ => panic!("inconsistent JPEG offset and length"),
    };
    let buf = exif.buf();
    Some(&buf[offset..offset + len])
}
