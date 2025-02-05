use std::sync::Arc;
use std::thread;
use crate::models::photo::Photo;
use crate::utils::img_util::ImageOperate;
use once_cell::sync::Lazy;
use crate::storage::connection::establish_connection;
use crate::storage::photo_table::insert_photo;
use crate::utils::task_util;
use std::sync::mpsc::{self, Sender, Receiver};
use diesel::SqliteConnection;
use rusqlite::{params, Connection};
use crate::storage::photo_table;


// pub static PHOTO_LOAD_RECEIVER1: Lazy<Arc<Sender<ImageOperate>>> =
//     Lazy::new(|| {
//         let (photo_handler_tx, photo_handler_rx) = mpsc::channel::<ImageOperate>();
//         let f = |io: ImageOperate| {
//             let mut conn = establish_connection();
//             insert_photo(&mut conn, io);
//         };
//         thread::spawn(move || {
//             for x in photo_handler_rx {
//
//             }
//         });
//
//         Arc::new(photo_handler_tx)
//     });

enum DbTask {
    PhotoBaseInsert(ImageOperate), // 插入任务：表名和数据
}


pub fn start_db_writer_thread(receiver: Receiver<DbTask>, conn: &mut SqliteConnection) {
    thread::spawn(move || {
        for task in receiver {
            match task {
                DbTask::PhotoBaseInsert(data) => {
                    let mut conn = establish_connection();
                    photo_table::insert_photo(&mut conn, data);
                    // if let Err(e) = insert_data(&conn, &table, data) {
                    //     eprintln!("Error inserting data: {}", e);
                    // }
                }
            }
        }
    });
}


/// 数据库状态管理
pub static PHOTO_LOAD_RECEIVER: Lazy<Arc<tauri::async_runtime::Sender<ImageOperate>>> =
    Lazy::new(|| {
        // 使用 tokio 的 mpsc 通道
        let (photo_handler_tx, photo_handler_rx) = tokio::sync::mpsc::channel::<ImageOperate>(100);
        let f = |io: ImageOperate| {
            let mut conn = establish_connection();
            insert_photo(&mut conn, io);
        };
        // 在一个新的线程中启动 Tokio 运行时
        thread::spawn(move || {
            // 创建 Tokio 运行时并运行异步任务
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async move {
                task_h(photo_handler_rx, f).await;
            });
        });

        Arc::new(photo_handler_tx)
    });

pub async fn task_h<T, F>(mut rx: tokio::sync::mpsc::Receiver<T>, f: F)
where
    F: Fn(T),
{
    // 无限循环确保 Receiver 一直保持活跃
    loop {
        match rx.recv().await {
            Some(task) => {
                f(task);
            }
            None => {
                // 如果通道关闭，可以选择如何处理（这里是等待并继续）
                println!("通道关闭，继续等待...");
            }
        }
    }
}
