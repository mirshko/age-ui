// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use age::{
    armor::{ArmoredWriter, Format},
    x25519, Encryptor,
};
use secrecy::{ExposeSecret};
use std::{
    io::{Write}, vec,
};

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

#[tauri::command]
fn encrypt_with_x25519(public_key: &str, data: &[u8]) -> Result<Box<[u8]>, String> {
    fn encrypt_error<T>(_: T) -> String {
        String::from("Encryption Error").into()
    }

    let key: x25519::Recipient = public_key.parse().map_err(encrypt_error)?;

    let recipients = vec![Box::new(key) as Box<dyn age::Recipient + Send>];

    let encryptor = Encryptor::with_recipients(recipients).ok_or_else(|| encrypt_error(""))?;

    let mut output = vec![];

    let armor =
        ArmoredWriter::wrap_output(&mut output, Format::AsciiArmor).map_err(encrypt_error)?;

    let mut writer = encryptor.wrap_output(armor).map_err(encrypt_error)?;

    writer.write_all(data).map_err(encrypt_error)?;

    writer
        .finish()
        .and_then(|armor| armor.finish())
        .map_err(encrypt_error)?;

    Ok(output.into_boxed_slice())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![greet, keygen, encrypt_with_x25519])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
