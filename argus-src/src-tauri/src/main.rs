// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 运行 Diesel 的 print-schema 命令
    // let output = Command::new("diesel")
    //     .args(&["print-schema"])
    //     .output()
    //     .expect("Failed to execute `diesel print-schema`");
    //
    //
    // // 将输出写入到 schema.rs
    // let path = "src/storage/schema1.rs";
    // std::fs::write(path, output.stdout)
    //     .expect(format!("Failed to write `{}`", path).as_str());

    argus_src_lib::run()
}
