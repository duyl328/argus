use tauri_plugin_dialog::DialogExt;
use crate::api::example::get_example;
use crate::http_client::HttpClient;
use crate::server;
use crate::server::example;

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

    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn http_example()  {
    // let example1 = example::http_get_example().await;

    let client = HttpClient::new();

    let example = get_example().await;
    match example{
        Ok(post) => println!("Fetched Post: {:?}", post),
        Err(err) => eprintln!("Error fetching post: {}", err),
    }
}
