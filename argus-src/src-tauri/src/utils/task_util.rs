use crate::storage::connection::establish_connection;
use crate::storage::photo::repository;
use crate::utils::img_util::ImageOperate;
use once_cell::sync::Lazy;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use crate::utils::exif_utils::tag::ImgExif;

/// 全局数据库任务管理
pub static DB_GLOBAL_TASK: Lazy<Arc<Sender<DbTask>>> = Lazy::new(|| {
    let (photo_handler_tx, photo_handler_rx) = mpsc::channel::<DbTask>();

    start_db_writer_thread(photo_handler_rx);

    Arc::new(photo_handler_tx)
});

pub enum DbTask {
    /// 插入基础图像信息（无 exif 信息）
    PhotoBaseInsert(ImageOperate),
    /// 插入图像完整信息（包含 exif 信息）
    PhotoFullInfoInsert(ImageOperate,ImgExif),
}

// todo: 2025/2/5 21:35 后期优化选项：根据任务条数，满足条数批量提交
// todo: 2025/2/5 21:39 将所有的 插入、修改、删除 操作都提交至此
pub fn start_db_writer_thread(receiver: Receiver<DbTask>) {
    let mut conn = establish_connection();
    thread::spawn(move || {
        for task in receiver {
            match task {
                DbTask::PhotoBaseInsert(data) => {
                    let result = repository::insert_photo(&mut conn, data);
                    if let Err(e) = result {
                        eprintln!("Error inserting data: {}", e);
                    }
                },
                DbTask::PhotoFullInfoInsert(data,exif) => {
                    let result = repository::insert_photo_and_info(&mut conn, data, exif);
                    if let Err(e) = result {
                        eprintln!("Error inserting data: {}", e);
                    }
                }
            }
        }
    });
}
