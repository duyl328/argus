#[tauri::command]
pub fn my_custom_command1() {
    println!("第二个可以调用的方法，但是这是无参数的");
}

#[tauri::command]
pub fn my_custom_command2(message: String) {
    println!("这是第三个调用的方法，这是有参数的：{}", message);
}


#[tauri::command]
pub fn my_custom_command3(message: String) -> String {
    println!("这是第四个调用的方法，这是有返回值：{}", message);
    message
}

#[tauri::command(async)]
pub fn my_custom_command4(message: String) -> Result<String, String> {
    if message == "tauri" {
        Ok("logged_in".to_string())
    } else {
        Err("密码错误".to_string())
    }
}


