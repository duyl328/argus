use std::env;

pub fn is_dev_util() -> bool {
    // let environment = env::var("TAURI_ENV").unwrap_or_else(|_| String::from("unknown"));
    let environment = env::var("ENV").unwrap_or_else(|_| String::from("development"));

    if environment == "development" {
        println!("Running in development environment");
    } else if environment == "production" {
        println!("Running in production environment");
    } else {
        println!("Unknown environment");
    }
    println!("environment {} !!!!!!!!!!",environment);
    return environment == "development";
}
