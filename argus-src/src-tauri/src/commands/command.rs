use crate::api::example::get_example;
use crate::http_client::HttpClient;
use crate::server;
use crate::server::example;
use tauri_plugin_dialog::DialogExt;
use crate::utils::exif_utils::exif_util;
use crate::utils::exif_utils::exif_util::ExifUtil;
use crate::utils::exif_utils::tag::Tags;

#[tauri::command]
pub fn greet(name: &str) -> String {
    println!("Hello, {}!", name);
    let builder = tauri::Builder::default();
    let res = builder.setup(|app| {
        println!("执行");
        app.dialog().message("Tauri is Awesome!").show(|_| {
            println!("dialog closed");
        });
        Ok(())
    });

    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn http_example() {
    // let example1 = example::http_get_example().await;

    let client = HttpClient::new();

    let example = get_example().await;
    match example {
        Ok(post) => println!("Fetched Post: {:?}", post),
        Err(err) => eprintln!("Error fetching post: {}", err),
    }
}

/// 读取图像 exif 信息
#[tauri::command]
pub async fn get_exif_info(path:String) -> Result<String, String> {
    let exif_tool = exif_util::ExifToolCmd;
    let exif_info = exif_tool.read_all_exif(&*path).expect("图像信息读取失败！");
    let mut tag = Tags::new(true);
    let mt = tag.parse(&exif_info);
    let result = mt.pack_front_tags().expect("数据打包失败！");
    Ok(result)
}
