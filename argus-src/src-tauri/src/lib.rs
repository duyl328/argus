mod api;
pub mod bg_services;
mod commands;
mod computed_value;
mod conf;
mod constant;
mod errors;
mod explore;
mod global_task_manager;
mod http_client;
mod models;
mod server;
mod services;
mod storage;
mod structs;
mod tuples;
mod utils;

use crate::storage::connection;
use crate::structs::config;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tauri::{async_runtime, AppHandle};

use crate::bg_services::{BgServes, SERVES};
use crate::global_task_manager::{start_image_loading_background_task, BackgroundTaskAutoManager};
use crate::storage::connection::establish_connection;
use crate::storage::photo_table::insert_photo;
use crate::structs::config::SYS_CONFIG;
use crate::utils::img_util::ImageOperate;
use crate::utils::task_util;
use tauri::{App, Emitter, Listener, Manager, State, WindowEvent};
use tokio::sync::{mpsc, watch};
use crate::utils::task_util::PHOTO_LOAD_RECEIVER;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        let devtools = tauri_plugin_devtools::init();
        builder = builder.plugin(devtools);
        // 调试日志集成至软件中
        // builder = builder.plugin(tauri_plugin_devtools::init()).plugin(tauri_plugin_devtools_app::init());
    }

    std::env::set_var("RUST_BACKTRACE", "1");
    // 非生产环境，使用外部日志
    #[cfg(not(debug_assertions))]
    {
        let log_plugin = tauri_plugin_log::Builder::new()
            // 设置文件大小
            .max_file_size(50_000)
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
            .level(log::LevelFilter::Info)
            // 仅对命令模块进行动词日志
            .level_for("commands::log_command", log::LevelFilter::Info)
            // 日志格式
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {}] - {}",
                    record.level(),
                    record.target(),
                    message
                ))
            })
            // 使用本机时间格式化日期
            .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
            .target(tauri_plugin_log::Target::new(
                // 将日志打印到终端
                // tauri_plugin_log::TargetKind::Stdout,
                // 记录到 webView
                // tauri_plugin_log::TargetKind::Webview,
                // 保存到文件
                /*
                Linux      {configDir}/{bundleIdentifier}                    /home/alice/.config/com.tauri.dev
                macOS      {homeDir}/Library/Logs/{bundleIdentifier}         /Users/Alice/Library/Logs/com.tauri.dev
                窗户	       {FOLDERID_LocalAppData}/{bundleIdentifier}/logs   C:\Users\Alice\AppData\Local\com.tauri.dev\logs
                    */
                // tauri_plugin_log::TargetKind::LogDir {
                //     file_name: Some("logs".to_string()),
                // },
                // 写入自定义位置
                tauri_plugin_log::TargetKind::Folder {
                    path: std::path::PathBuf::from(constant::LOG_PATH),
                    file_name: None,
                },
            ))
            .build();
        builder = builder.plugin(log_plugin)
    }

    // 初始化配置文件
    let configs = config::init_config();

    print!("配置完毕!!!!!!!!!!!!!");

    // 启动后台算法
    // 创建一个大小为 100 的通道
    let (tx, rx) = mpsc::channel::<String>(100);
    let (pause_tx, pause_rx) = watch::channel(false); // 创建暂停信号
    let (auto_manager_tx, auto_manager_rx) = watch::channel(BackgroundTaskAutoManager::default());

    // 启动 Tokio 运行时
    async_runtime::spawn(async {
        // 后台图像加载
        start_image_loading_background_task(rx, pause_rx, auto_manager_rx).await;
        // 数据库后台写入
    });

    let global_task_manager = tokio::sync::Mutex::new(
        global_task_manager::BackgroundImageLoadingTaskManager::new(tx, pause_tx, auto_manager_tx),
    );

    // 全局句柄
    let app_handle = Arc::new(tokio::sync::Mutex::new(None::<AppHandle>));

    // 启动后台服务
    back_a_task();
    builder
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .on_window_event(|windows, event| {
            // 事件处理
            match event {
                // 窗口关闭事件
                WindowEvent::CloseRequested { api, .. } => {
                    println!("进入关闭流程！ ");
                    SERVES.write().unwrap().drop_all();
                }
                _ => {}
            }
        })
        // 报错: state not managed for field state on command XXX. You must call .manage() before using this command
        // Tauri 通过类型来管理和注入状态，因此在 .manage() 中注册的类型必须是唯一的。
        // 如果你尝试注册同一个类型多次，Tauri 会抛出错误。
        // 使用时一定要注意类型一定要一致 !!!
        .manage::<Option<tauri_plugin_shell::process::CommandChild>>(None)
        .manage(global_task_manager)
        .manage(app_handle)
        .invoke_handler(tauri::generate_handler![
            commands::command::greet,
            commands::command::http_example,
            commands::command::get_exif_info,
            commands::file_command::get_image_absolute_path,
            commands::file_command::check_directory_access,
            commands::file_command::read_image_as_base64,
            commands::file_command::get_all_sub_dir,
            commands::file_command::get_all_imgs,
            commands::file_command::get_dir_all_subfolders_first_img,
            commands::post_command::get_all_post,
            commands::post_command::insert_post,
            commands::log_command::log_logs,
            commands::emit_test::emit_send_test,
            commands::photo_storage_command::get_photo_storage,
            commands::photo_storage_command::add_photo_storage,
            commands::photo_storage_command::delete_photo_storage,
            commands::photo_storage_command::update_photo_storage,
            commands::folder_show_command::get_need_display_image_info,
            commands::image_command::get_compress_image_address,
            commands::image_command::generate_save_thumbnail,
            commands::image_command::get_image_thumbnail_path,
            commands::image_command::get_image_thumbnail,
            commands::global_task_command::add_task,
            commands::global_task_command::pause_task,
            commands::global_task_command::resume_task,
            commands::global_task_command::set_app_handle,
        ])
        .setup(main_setup())
        .run(tauri::generate_context!())
        .expect("argus 启动失败!");
}

fn main_setup() -> fn(&mut App) -> Result<(), Box<dyn Error>> {
    |app| {
        log::info!(" =============================== 程序启动！==============================");
        #[cfg(not(debug_assertions))]
        {
            log::info!("{}", constant::BANNER4);
        }

        let cpath = std::env::current_dir().expect("软件路径获取");
        log::info!("软件路径:{:?}", cpath);

        log::info!("创建数据库");
        let db = connection::run_migrations().expect("Database initialize should succeed");
        log::info!("创建完毕");

        // 创建指定目录
        let lazy = SYS_CONFIG.thumbnail_storage_path.clone().unwrap();
        println!("输出的路径：{}", lazy);

        // 启动服务
        // `sidecar()` 只需要文件名, 不像 JavaScript 中的整个路径
        println!("sidecar().");

        // 启用 python 算法
        bg_services::start_python_service().unwrap();
        let app_handle = app.handle();

        // 打开控制台
        #[cfg(debug_assertions)] // 仅在调试版本中包含此代码
        {
            let window = app.get_webview_window("main").unwrap();
            window.open_devtools();
            window.close_devtools();
        }

        Ok(())
    }
}

/// 后台服务
fn back_a_task() {
    println!("后台服务 初始化");
    let sender = PHOTO_LOAD_RECEIVER.clone();
    let lazy = PHOTO_LOAD_RECEIVER.clone();
    
    // use tokio::sync::{mpsc, Mutex};  // 需要引入 mpsc 和 Mutex
    // 
    // 
    // // 使用 tokio 的 mpsc 通道
    // let (photo_handler_tx, photo_handler_rx) = mpsc::channel::<ImageOperate>(100);
    // 
    // // 使用 .await 获取 Mutex 锁
    // let mut photo_handler:tauri::async_runtime::Sender<ImageOperate> = task_util::PHOTO_LOAD_RECEIVER.lock().await.unwrap();
    // 
    // // 修改 photo_handler 中的值
    // *photo_handler = Some(photo_handler_tx);
    // 
    // // Define the function to process ImageOperate
    // let f = |io: ImageOperate| {
    //     let mut conn = establish_connection();
    //     // Read information and save to the database
    //     insert_photo(&mut conn, io);
    // };
    // 
    // // Start the background task
    // task_util::task_h(photo_handler_rx, f);
}