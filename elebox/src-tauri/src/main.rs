// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use elebox_core::{self, Part, PartType};
use std::sync::Mutex;

const DEFAULT_DB_PATH: &str = "./elebox.db";
struct DbPath(Mutex<String>);

#[tauri::command]
fn get_parts(db: tauri::State<DbPath>) -> Vec<Part> {
    elebox_core::get_parts(&db.0.lock().unwrap())
}

#[tauri::command]
fn part_del(db: tauri::State<DbPath>, part: &str) {
    elebox_core::delete_part(&db.0.lock().unwrap(), &part.to_string());
}

#[tauri::command]
fn part_add(db: tauri::State<DbPath>, part: &str, qty: i16) {
    elebox_core::update_part_quantity(&db.0.lock().unwrap(), &part.to_string(), qty);
}

#[tauri::command]
fn type_new(db: tauri::State<DbPath>, name: &str, parent: &str) {
    elebox_core::add_part_type(
        &db.0.lock().unwrap(),
        &name.to_string(),
        Some(&parent.to_string()),
    );
}

#[tauri::command]
fn part_new(db: tauri::State<DbPath>, name: &str, qty: u16, ptype: &str) {
    elebox_core::add_part(
        &db.0.lock().unwrap(),
        &name.to_string(),
        &qty,
        &ptype.to_string(),
    );
}

#[tauri::command]
fn get_types(db: tauri::State<DbPath>) -> Vec<PartType> {
    elebox_core::get_part_types(&db.0.lock().unwrap())
}

fn main() {
    tauri::Builder::default()
        .manage(DbPath(Mutex::new(DEFAULT_DB_PATH.to_string())))
        .invoke_handler(tauri::generate_handler![
            get_parts, get_types, part_add, part_new, part_del, type_new
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn update_db_path(db: tauri::State<DbPath>, new_path: &str) {
    let mut p = db.0.lock().unwrap();
    *p = String::from(new_path);
}
