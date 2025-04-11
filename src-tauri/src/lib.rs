// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


use rand::prelude::*;

#[derive(Debug, serde::Deserialize)]
struct PasswordOptions {
    length: usize,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
}

#[tauri::command]
fn generate_password(params: PasswordOptions) -> Result<String, String> {
   
    if params.length < 4 {
        return Err("Пароль не может быть короче  4 символов".into());
    }

    let mut charset = String::new();
    if params.uppercase { charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
    if params.lowercase { charset.push_str("abcdefghijklmnopqrstuvwxyz"); }
    if params.numbers { charset.push_str("0123456789"); }
    if params.symbols { charset.push_str("!@#$%^&*"); }

    if charset.is_empty() {
        return Err("Должны быть выбраны хотябы какие-то текстовые характеристики".into());
    }

    let password: String = (0..params.length)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    Ok(password)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
                   
            generate_password 
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
