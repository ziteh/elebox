// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs;
use elebox_core::{Part, PartType};
use serde::de::value::Error;
use std::path::PathBuf;
use std::sync::Mutex;

macro_rules! DB_PATH {
    ($db:expr) => {
        $db.0.lock().unwrap()
    };
}

struct DbPath(Mutex<String>);

#[tauri::command]
fn get_parts(db: tauri::State<DbPath>) -> Vec<Part> {
    Part::list(&DB_PATH!(db))
}

#[tauri::command]
fn part_del(db: tauri::State<DbPath>, part: &str) {
    let _ = Part::delete_by_name(&DB_PATH!(db), &part.to_string());
}

#[tauri::command]
fn part_add(db: tauri::State<DbPath>, part: &str, qty: i16) {
    let _ = Part::update_part_quantity(&DB_PATH!(db), &part.to_string(), qty);
}

#[tauri::command]
fn type_new(db: tauri::State<DbPath>, name: &str, parent: &str) {
    let pt = PartType {
        name: name.to_string(),
        parent: Some(parent.to_string()),
    };
    let _ = pt.add(&DB_PATH!(db));
}

#[tauri::command]
fn part_new(db: tauri::State<DbPath>, name: &str, qty: u16, ptype: &str) {
    let part = Part::new(name, ptype, qty);
    let _ = part.add(&DB_PATH!(db));
}

#[tauri::command]
fn get_types(db: tauri::State<DbPath>) -> Vec<PartType> {
    PartType::list(&DB_PATH!(db))
}

#[tauri::command]
fn get_db_path(db: tauri::State<DbPath>) -> String {
    DB_PATH!(db).to_string()
}

#[tauri::command]
fn set_db_path(db: tauri::State<DbPath>, path: &str) {
    update_db_path(db, path);
}

fn main() {
    let db_path = match get_default_path() {
        Ok(path) => path,
        Err(err) => panic!("{}", err),
    };

    init_db(&db_path);

    tauri::Builder::default()
        .manage(DbPath(Mutex::new(db_path)))
        .invoke_handler(tauri::generate_handler![
            get_parts,
            get_types,
            part_add,
            part_new,
            part_del,
            type_new,
            get_db_path,
            set_db_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_default_path() -> Result<String, String> {
    if let Some(mut dir) = dirs::data_local_dir() {
        dir.push("elebox");

        if let Err(err) = std::fs::create_dir_all(&dir) {
            return Err(format!("Unable to creating directory. {}", err));
        }

        let db_path = dir.join("elebox.db");
        return Ok(db_path.into_os_string().into_string().unwrap());
    } else {
        return Err("Unable to determine user's local data directory.".to_string());
    }
}

fn update_db_path(db: tauri::State<DbPath>, new_path: &str) {
    let mut p = DB_PATH!(db);
    *p = String::from(new_path);
}

fn init_db(path: &str) {
    elebox_core::init(path);
}
