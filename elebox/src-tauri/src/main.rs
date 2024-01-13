// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use elebox_core::{Part, PartType};
use std::sync::Mutex;

const DEFAULT_DB_PATH: &str = "./elebox.db";
struct DbPath(Mutex<String>);

#[tauri::command]
fn get_parts(db: tauri::State<DbPath>) -> Vec<Part> {
    Part::list(&db.0.lock().unwrap())
}

#[tauri::command]
fn part_del(db: tauri::State<DbPath>, part: &str) {
    let _ = Part::delete_by_name(&db.0.lock().unwrap(), &part.to_string());
}

#[tauri::command]
fn part_add(db: tauri::State<DbPath>, part: &str, qty: i16) {
    let _ = Part::update_part_quantity(&db.0.lock().unwrap(), &part.to_string(), qty);
}

#[tauri::command]
fn type_new(db: tauri::State<DbPath>, name: &str, parent: &str) {
    let pt = PartType {
        name: name.to_string(),
        parent: Some(parent.to_string()),
    };
    let _ = pt.add(&db.0.lock().unwrap());
}

#[tauri::command]
fn part_new(db: tauri::State<DbPath>, name: &str, qty: u16, ptype: &str) {
    let part = Part::new(name, ptype, qty);
    let _ = part.add(&db.0.lock().unwrap());
}

#[tauri::command]
fn get_types(db: tauri::State<DbPath>) -> Vec<PartType> {
    PartType::list(&db.0.lock().unwrap())
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
