mod commands;
mod constant;
mod models;
mod services;
mod storage;
mod utils;
mod errors;

use tauri::{Emitter, Listener, Manager, State};
use tauri_plugin_sql::{Migration, MigrationKind};
use crate::storage::connection;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::env::set_var("RUST_BACKTRACE", "1");
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
        )).build();


    // let migrations = vec![
    //     Migration {
    //         version: 1,
    //         description: "create users table",
    //         sql: "CREATE TABLE IF NOT EXISTS users (
    //             id INTEGER PRIMARY KEY AUTOINCREMENT,
    //             name TEXT NOT NULL,
    //             email TEXT
    //         )",
    //         kind: MigrationKind::Up,
    //     }
    // ];


    tauri::Builder::default()
        .plugin(log_plugin)
        .plugin(tauri_plugin_sql::Builder::new()
            .build()
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::command::greet,
            commands::file_command::get_image_absolute_path,
            commands::file_command::check_directory_access,
            commands::file_command::read_image_as_base64,
            commands::post_command::get_all_post,
            commands::post_command::insert_post,
            commands::log_command::log_logs,
            commands::photo_storage_command::get_photo_storage,
            commands::photo_storage_command::add_photo_storage,
            commands::photo_storage_command::delete_photo_storage,
            commands::photo_storage_command::update_photo_storage,
        ])
        .setup(|app| {
            log::info!(" =============================== 程序启动！==============================");
            log::info!("{}", constant::BANNER4);
            let cpath = std::env::current_dir().expect("TODO: panic message");
            let spath = std::env::current_exe().expect("TODO: panic message");
            log::info!("软件路径:{:?}",cpath);
            log::info!("软件路径1:{:?}",spath);


            let handle = app.handle();
            log::info!("创建数据库");
            // let db = connection::init_database().expect("Database initialize should succeed");
            let db = connection::run_migrations().expect("Database initialize should succeed");
            log::info!("创建完毕");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("argus 启动失败!");
}
