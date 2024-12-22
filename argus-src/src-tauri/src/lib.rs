mod commands;
mod constant;
mod errors;
mod explore;
mod models;
mod services;
mod storage;
mod structs;
mod utils;

use crate::storage::connection;
use crate::structs::config;
use crate::structs::config::SYS_CONFIG;
use crate::utils::file_util::create_folder;
use tauri::{Emitter, Listener, Manager, State};
use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        let devtools = tauri_plugin_devtools::init();
        builder = builder.plugin(devtools);
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

    builder
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::command::greet,
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
        ])
        .setup(|app| {
            log::info!(" =============================== 程序启动！==============================");
            #[cfg(debug_assertions)]
            {
                log::info!("{}", constant::BANNER4);
            }
            // 初始化配置文件
            let configs = config::init_config();

            let cpath = std::env::current_dir().expect("软件路径获取");
            log::info!("软件路径:{:?}", cpath);

            log::info!("创建数据库");
            let db = connection::run_migrations().expect("Database initialize should succeed");
            log::info!("创建完毕");

            // 创建指定目录
            let lazy = configs.thumbnail_storage_path.unwrap();
            println!("输出的路径：{}", lazy);

            // 打开控制台
            #[cfg(debug_assertions)] // 仅在调试版本中包含此代码
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("argus 启动失败!");
}
