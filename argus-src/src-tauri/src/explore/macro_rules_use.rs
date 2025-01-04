



macro_rules! define_struct {
    // 生成结构体的宏
    ($struct_name:ident, { $($field_name:ident: $field_type:ty, $description:expr),* }) => {
        pub struct $struct_name {
            // 为结构体生成字段
            $(
                pub $field_name: $field_type,
            )*
        }

        impl $struct_name {
            // 生成方法来打印字段的描述
            pub fn describe_fields() {
                $(
                    println!("{}: {}", stringify!($field_name), $description);
                )*
            }
        }
    };
}

mod tests {
    use super::*;

    #[test]
    fn test_define_struct() {
        define_struct!(Person, {
            name: String, "The name of the person",
            age: u32, "The age of the person",
            email: String, "The email address of the person"
        });
    }
}
