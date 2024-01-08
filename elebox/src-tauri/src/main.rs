// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use elebox_core::{self, Part, PartType};

#[tauri::command]
fn get_parts() -> Vec<Part> {
    elebox_core::get_parts(&String::from("./elebox.db"))
}

#[tauri::command]
fn part_add(part: &str, qty: i16) {
    elebox_core::update_part_quantity(&String::from("./elebox.db"), &part.to_string(), qty);
}

#[tauri::command]
fn part_new(name: &str, qty: u16, ptype: &str) {
    elebox_core::add_part(
        &String::from("./elebox.db"),
        &name.to_string(),
        &qty,
        &ptype.to_string(),
    );
}

#[tauri::command]
fn get_types() -> Vec<PartType> {
    elebox_core::get_part_types(&String::from("./elebox.db"))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_parts, get_types, part_add, part_new
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
