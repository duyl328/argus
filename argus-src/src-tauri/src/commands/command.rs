use tauri_plugin_dialog::DialogExt;

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

    println!();

    format!("Hello, {}! You've been greeted from Rust!", name)
}
