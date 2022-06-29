#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};

#[tauri::command]
fn simple_command() {
    println!("simple command");
}

#[tauri::command]
fn command_with_message(msg: String) -> String {
    format!("Hello {}", msg)
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    field_str: String,
    field_u32: u32,
}

#[tauri::command]
fn command_with_object(msg: Message) -> Message {
    let Message {
        field_str,
        field_u32,
    } = msg;

    Message {
        field_str: format!("Hello {}", field_str),
        field_u32: field_u32 + 1,
    }
}

#[tauri::command]
fn command_with_error(arg: u32) -> Result<String, String> {
    if arg % 2 == 0 {
        Ok(format!("even value {}", arg))
    } else {
        Err(format!("odd value {}", arg))
    }
}

#[tauri::command]
fn async_command(arg: u32) -> String {
    "hello".into()
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            simple_command,
            command_with_message,
            command_with_object,
            command_with_error,
            async_command,
        ])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
