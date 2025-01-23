use crate::constant::{IMAGE_COMPRESSION_RATIO, IMAGE_COMPRESSION_STORAGE_FORMAT};
use crate::global_front_emit;
use crate::structs::global_error_msg::{
    GlobalErrorMsg, LoadMsg, GLOBAL_EMIT_APP_HANDLE, GLOBAL_EMIT_IS_INIT, IMG_DISPOSE_IS_CANCEL,
    IMG_DISPOSE_IS_START,
};
use crate::tuples::Pair;
use crate::utils::file_util::{get_all_dir_img, get_all_subfolders};
use crate::utils::img_util::ImageOperate;
use crate::utils::json_util::JsonUtil;
use crate::utils::task_util::task_h;
use anyhow::Result;
use std::sync::{Arc, RwLock};
use std::thread;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, Semaphore};
use tokio::task;
use crate::utils::exif_utils::exif_util;
use crate::utils::exif_utils::exif_util::ExifUtil;
use crate::utils::exif_utils::tag::Tags;

#[tauri::command]
pub async fn add_photo_retrieve_task(
    app: AppHandle,
    tasks: Vec<String>,
    is_cancel: bool,
) -> Result<String, String> {
    // 任务是否取消
    let mut is_cc = IMG_DISPOSE_IS_CANCEL.lock().await;
    *is_cc = is_cancel;

    println!("add_task: {:?}", tasks);
    // 获取指定路径下所有的文件
    let mut result: Vec<String> = Vec::new();
    for x in tasks {
        let vec = get_all_subfolders(&x);
        // 使用并发处理文件夹
        for x in &vec {
            let display = x.display().to_string();
            // 获取所有照片
            let vec1 = get_all_dir_img(&display, Some(-1)); // 获取文件夹中的图像路径
            if !vec1.is_empty() {
                result.extend(vec1)
            }
        }
    }

    // 总任务数
    let lens = result.clone().len();
    // 当前任务数
    let data = Arc::new(RwLock::new(0));
    // 最多 10 个任务
    let semaphore = Arc::new(Semaphore::new(20)); // 最多 10 个任务同时执行
     // 添加任务
    for x in result {
        let data = Arc::clone(&data);
        let ap = app.clone();
        let permit = Arc::clone(&semaphore);
        task::spawn(async move {
            let _permit = permit.acquire().await.unwrap(); // 等待获取一个令牌
            let is_cc = *IMG_DISPOSE_IS_CANCEL.lock().await;
            if is_cc {
                return;
            }

            // 压缩图像
            let image_compression = ImageOperate::multi_level_image_compression(
                x.clone(),
                IMAGE_COMPRESSION_STORAGE_FORMAT,
                IMAGE_COMPRESSION_RATIO.to_vec(),
            );
            
            // 获取 exif 
            // let exif_tool = exif_util::ExifToolCmd;
            // let exif_info = exif_tool.read_all_exif(&*x).expect("图像信息读取失败！");
            // let tag = Tags::new(true);
            // let mt = tag.parse(&exif_info);
            // let result = mt.pack_object().expect("数据打包失败！");
            // 

            let result1 = image_compression.await;

            let mut num = data.write().unwrap(); // 获取写锁
            *num += 1;
            let s = *num;
            match result1 {
                Ok(_) => {
                    let lm = LoadMsg {
                        all_task: lens as u32,
                        current_task: s,
                        task_msg: x,
                    };
                    let str = JsonUtil::stringify(&lm).unwrap();

                    ap.emit(global_front_emit::PHOTO_LOADING_MSG_TIP, str)
                        .unwrap();
                }
                Err(e) => {
                    // 将错误传递到主线程
                    let lm = LoadMsg {
                        all_task: lens as u32,
                        current_task: s,
                        task_msg: x,
                    };
                    let str = JsonUtil::stringify(&lm).unwrap();

                    ap.emit(global_front_emit::PHOTO_LOADING_MSG_TIP, str)
                        .unwrap();

                    ap.emit(
                        global_front_emit::PHOTO_LOADING_ERR_TIP,
                        format!("{} 出错: {}", lm.task_msg, e.to_string()),
                    )
                    .unwrap();
                }
            }
        });
    }

    Ok(String::from("完成"))
}

#[tauri::command]
pub fn emit_global_msg(app: AppHandle) {
    let mut is_init = GLOBAL_EMIT_IS_INIT.lock().unwrap();
    if is_init.clone() {
        return;
    }
    println!("前端 emit 初始化!");
    *is_init = true;

    let mut emit = GLOBAL_EMIT_APP_HANDLE.lock().unwrap();
    let (emit_tx, emit_rx) = mpsc::channel::<String>(100);
    *emit = Some(emit_tx);
    let f = move |info: String| {
        app.clone()
            .emit(global_front_emit::GLOBAL_ERROR_MSG_DISPLAY, info)
            .unwrap();
    };

    // 在一个新的线程中启动 Tokio 运行时
    thread::spawn(move || {
        // 创建 Tokio 运行时并运行异步任务
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            task_h(emit_rx, f).await;
        });
    });
}

#[tauri::command]
pub async fn global_msg_emit() -> Result<String, String> {
    let emit_option = {
        let emit = GLOBAL_EMIT_APP_HANDLE.lock().unwrap();
        emit.clone() // 移出作用域以释放锁
    };

    if let Some(x) = emit_option.as_ref() {
        let ms = GlobalErrorMsg {
            title: "标题".parse().unwrap(),
            msg: "内容".parse().unwrap(),
            duration: 5000,
            kind: crate::structs::global_error_msg::GlobalErrorMsgTypeEnum::Success,
        };
        let result = JsonUtil::stringify(&ms).expect("数据序列化失败!");
        let qqq = x.send(result).await;
        if qqq.is_err() {
            println!("发送失败！");
        }
    }
    return Ok(String::from("Ok"));
}
