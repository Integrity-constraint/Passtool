// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


use rand::prelude::*;
use std::sync::Mutex;
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use rusqlite::{params, Connection, Result};

#[macro_use]
extern crate lazy_static;


#[derive(Debug, serde::Deserialize)]
struct PasswordOptions {
    length: usize,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
}

#[derive(Debug, serde::Deserialize)]
struct Resource_Passwords{
Res_name: String,
The_password: String
}

lazy_static! {
static ref PASS_TRANSFER: Mutex<String> = Mutex::new(String::new());
}


#[tauri::command]
fn generate_password(params: PasswordOptions) -> Result<String, String> {
   
    if params.length < 4 {
        return Err("Пароль не может быть короче  4 символов".into());
    }
    if params.length > 100 {
        return Err("слишком много символов".into());
    }
    let mut charset = String::new();
    if params.uppercase { charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
    if params.lowercase { charset.push_str("abcdefghijklmnopqrstuvwxyz"); }
    if params.numbers { charset.push_str("0123456789"); }
    if params.symbols { charset.push_str("!@#$%^&*"); }

    if charset.is_empty() {
        return Err("Должны быть выбраны хотябы какие-то текстовые характеристики".into());
    }

    let mut  password: String = (0..params.length)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();
  
        let mut global_pass = PASS_TRANSFER.lock().unwrap();
        *global_pass = password.clone(); 
        
        Ok(password)
    
}

#[tauri::command]
fn get_last_password_copy() -> Option<String> {
    Some(PASS_TRANSFER.lock().unwrap().clone())
}
#[tauri::command]
async fn save_to_file() -> Result<(), String> {
    let pass = get_last_password_copy()
        .ok_or("Нет пароля для сохранения".to_string())?;
    
    if pass.is_empty() {
        return Err("Пароль пуст".to_string());
    }

   
    
    let file_path =  Path::new("saved_password.txt");

   
    File::create(&file_path)
        .map_err(|e| format!("Не удалось создать файл: {}", e))?
        .write_all(pass.as_bytes())
        .map_err(|e| format!("Не удалось провести запись в файл: {}", e))?;

    Ok(())
}

#[tauri::command]
fn add_entry(resource: String, password: String) -> Result<(), String> {
    let conn = Connection::open("pass_tool.db").map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO Passwords (Res_name, the_password) VALUES (?1, ?2)",
        [&resource, &password],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
#[tauri::command]
fn create_bd() -> Result<(), String> {
    let conn = Connection::open("pass_tool.db").map_err(|e| e.to_string())?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Passwords (
            id INTEGER PRIMARY KEY,
            Res_name TEXT NOT NULL,
            the_password TEXT NOT NULL
        )",
        [],
    ).map_err(|e| e.to_string())?;

    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
          
            generate_password,
            get_last_password_copy,
            save_to_file, create_bd
          
          
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
