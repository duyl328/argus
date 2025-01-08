
extern crate exif;

use std::fs::File;
use std::io::BufReader;

use exif::{DateTime, In, Reader, Value, Tag};

fn main() {
    let file = File::open("tests/exif.jpg").unwrap();
    let exif = Reader::new().read_from_container(
        &mut BufReader::new(&file)).unwrap();

    // To obtain a string representation, `Value::display_as`
    // or `Field::display_value` can be used.  To display a value with its
    // unit, call `with_unit` on the return value of `Field::display_value`.
    let tag_list = [Tag::ExifVersion,
                    Tag::PixelXDimension,
                    Tag::XResolution,
                    Tag::ImageDescription,
                    Tag::DateTime];
    for tag in tag_list {
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            println!("{}: {}",
                     field.tag, field.display_value().with_unit(&exif));
        }
    }

    // To get unsigned integer value(s) from either of BYTE, SHORT,
    // or LONG, `Value::get_uint` or `Value::iter_uint` can be used.
    if let Some(field) = exif.get_field(Tag::PixelXDimension, In::PRIMARY) {
        if let Some(width) = field.value.get_uint(0) {
            println!("Valid width of the image is {}.", width);
        }
    }

    // To convert a Rational or SRational to an f64, `Rational::to_f64`
    // or `SRational::to_f64` can be used.
    if let Some(field) = exif.get_field(Tag::XResolution, In::PRIMARY) {
        match field.value {
            Value::Rational(ref vec) if !vec.is_empty() =>
                println!("X resolution is {}.", vec[0].to_f64()),
            _ => {},
        }
    }

    // To parse a DateTime-like field, `DateTime::from_ascii` can be used.
    if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
        match field.value {
            Value::Ascii(ref vec) if !vec.is_empty() => {
                if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
                    println!("Year of DateTime is {}.", datetime.year);
                }
            },
            _ => {},
        }
    }
}
