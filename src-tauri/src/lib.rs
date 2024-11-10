// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs::File;
use std::io::Write;

#[tauri::command]
fn greet(name: &str) -> String {
    print!("it works {}" , name);
    let mut file = File::create(name).unwrap();
    file.write_all(b"Hello, Rust!").unwrap();
    format!("Hello, {} you have written a file from rust", name) 
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
