// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_files(parent_dir: &str, show_hidden: bool) -> Vec<String> {
    let dir = Path::new(parent_dir);

    let mut dir_vector = Vec::new();

    if parent_dir != "/" {
        dir_vector.push(String::from("../"));
    }

    if !dir.is_dir() {
        return dir_vector;
    }

    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        if !show_hidden
            && path
                .as_ref()
                .unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(".")
        {
            continue;
        }

        if path.as_ref().unwrap().path().is_dir() {
            dir_vector.push(format!(
                "{}/",
                path.unwrap().path().file_name().unwrap().to_str().unwrap()
            ));
        } else {
            dir_vector.push(String::from(
                path.unwrap().path().file_name().unwrap().to_str().unwrap(),
            ));
        }
    }

    return dir_vector;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
