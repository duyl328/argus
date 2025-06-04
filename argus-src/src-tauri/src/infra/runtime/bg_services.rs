use crate::utils::file_util;
use anyhow::Result;
use sha2::digest::impl_oid_carrier;
use std::env;
use std::process::{Child, Command, Output};
use std::sync::{Arc, RwLock};
use tauri::App;
use tauri_plugin_shell::ShellExt;
use thiserror::__private::AsDisplay;
use crate::utils::env_util::is_dev_util;

pub struct BgServes {
    pub py_service_id: Option<u32>,
}

impl BgServes {
    pub fn drop_all(&self) {
        if is_dev_util() { return; }
        let id = self.py_service_id.expect("后台 ID 获取失败！");
        println!("主程序退出，杀掉子进程...");
        if cfg!(unix) {
            let _ = Command::new("kill")
                .arg("-9") // 强制终止
                .arg(id.to_string())
                .spawn()
                .expect("无法杀死进程");
        } else if cfg!(windows) {
            let _ = Command::new("taskkill")
                .arg("/F") // 强制终止
                .arg("/PID")
                .arg(id.to_string())
                .spawn()
                .expect("无法杀死进程");
        }
    }
}

pub(crate) static SERVES: once_cell::sync::Lazy<Arc<RwLock<BgServes>>> =
    once_cell::sync::Lazy::new(|| {
        Arc::new(RwLock::new(BgServes {
            py_service_id: None,
        }))
    });

/// 启动 python 服务
pub fn start_python_service() -> Result<()> {
    if is_dev_util() { return Ok(()); }
    let current_exe_path = env::current_exe()?;
    // 获取当前程序目录
    let current_dir = current_exe_path.parent().expect("父路径获取！");
    // 拼接 resevice/app.exe 的路径
    let app_path = current_dir.join("service").join("argus-app.exe");

    // 文件是否存在
    let string = app_path.clone().display().to_string();
    println!("准备启动服务 {}", string);
    if !file_util::file_exists(&*string) {
        return Err(anyhow::anyhow!("服务文件不存在！"));
    }
    println!("开始运行后台服务");
    // 启动 app.exe
    let mut output = Command::new(app_path)
        // .arg()
        .current_dir(current_dir.join("service")) // 设置工作目录到服务目录
        .spawn() // 非阻塞方式启动子进程
        .expect("无法启动 Flask 服务");
    // output.kill();
    // 将子进程的句柄存储起来，用于在主程序退出时终止它
    let child_pid = output.id();
    println!("子进程 PID: {}", child_pid);
    SERVES.write().expect("服务信息读取失败！").py_service_id = Some(child_pid);
    Ok(())
}

pub fn stop_python_service() {
    // let mut guard = SERVES.write().expect("服务信息读取失败！");
    // let child = guard.py_service.expect("信息获取失败");
}
