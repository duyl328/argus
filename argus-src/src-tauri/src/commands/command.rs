use crate::api::example::get_example;
use crate::utils::exif_utils::exif_util;
use crate::utils::exif_utils::exif_util::ExifUtil;
use crate::utils::exif_utils::tag::Tags;
use tauri_plugin_dialog::DialogExt;
use crate::net_connection::http_client::HttpClient;

#[tauri::command]
pub fn greet(name: &str) -> String {
    println!("Hello, {}!", name);
    let builder = tauri::Builder::default();
    let _res = builder.setup(|app| {
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

    let _client = HttpClient::new();

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
    let tag = Tags::new(true);
    let mt = tag.parse(&exif_info);
    let result = mt.pack_front_tags().expect("数据打包失败！");
    Ok(result)
}

/// 获取前端可使用的内存大小
pub fn get_front_can_use_memory(){
    // todo: 2025/4/7 17:35 获取对应可使用的内存量，可使用总内存的百分之 10 ，然后计算临近的整数内存
    let (fm,am) = crate::utils::system_state_util::get_memory();
    
}


// 全局异常通知
#[tauri::command]
pub fn global_exception_notifications(){
    
}

