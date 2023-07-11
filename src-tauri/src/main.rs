// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use age::x25519;
use secrecy::ExposeSecret;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn keygen() -> Result<Vec<String>, String> {
    let secret = x25519::Identity::generate();
    let public = secret.to_public();
    Ok(vec![
        String::from(secret.to_string().expose_secret()),
        String::from(public.to_string()),
    ])
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![greet, keygen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
