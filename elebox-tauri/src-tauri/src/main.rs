// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs;
use elebox_core::{Category, Datebase, Part};
use std::sync::Mutex;
use tauri::Manager;

macro_rules! GET {
    ($db:expr) => {
        $db.0.lock().unwrap()
    };
}

struct DbPath(Mutex<String>);

#[tauri::command]
fn get_parts(path: tauri::State<DbPath>) -> Vec<Part> {
    let db = elebox_core::JammDatebase::new(&GET!(path));
    let mgr = elebox_core::PartManager::new(&db);
    mgr.list()
}

#[tauri::command]
fn part_del(path: tauri::State<DbPath>, part: &str) {
    let db = elebox_core::JammDatebase::new(&GET!(path));
    let mgr = elebox_core::PartManager::new(&db);
    let _ = mgr.delete(part);
}

#[tauri::command]
fn part_add(path: tauri::State<DbPath>, part: &str, qty: i16) {
    let db = elebox_core::JammDatebase::new(&GET!(path));
    let mgr = elebox_core::PartManager::new(&db);
    let _ = mgr.update_part_quantity(part, qty);
}

#[tauri::command]
fn new_category(path: tauri::State<DbPath>, name: &str, parent: &str) {
    let db = elebox_core::JammDatebase::new(&GET!(path));
    let mgr = elebox_core::CategoryManager::new(&db);

    let cat = Category {
        name: name.to_string(),
        parent: match parent {
            "" => None,
            _ => Some(parent.to_string()),
        },
    };
    let _ = mgr.add(&cat);
}

#[tauri::command]
fn del_category(path: tauri::State<DbPath>, name: &str) -> String {
    let db = elebox_core::JammDatebase::new(&GET!(path));
    let mgr = elebox_core::CategoryManager::new(&db);

    let res = mgr.delete(name);
    match res {
        Err(e) => return e.to_string(),
        Ok(s) => return format!("OK {s}"),
    }
}

#[tauri::command]
fn part_new(path: tauri::State<DbPath>, name: &str, qty: u16, ptype: &str) {
    let db = elebox_core::JammDatebase::new(&GET!(path));
    let mgr = elebox_core::PartManager::new(&db);
    let part = Part::new(name, ptype, qty);
    let _ = mgr.add(&part);
}

#[tauri::command]
fn get_categories(path: tauri::State<DbPath>) -> Vec<Category> {
    let db = elebox_core::JammDatebase::new(&GET!(path));
    let mgr = elebox_core::CategoryManager::new(&db);
    mgr.list()
}

#[tauri::command]
fn get_db_path(path: tauri::State<DbPath>) -> String {
    GET!(path).to_string()
}

#[tauri::command]
fn set_db_path(path: tauri::State<DbPath>, new_path: &str) {
    update_db_path(path, new_path);
    init_db(new_path);
}

fn main() {
    let db_path = match get_default_path() {
        Ok(path) => path,
        Err(err) => panic!("{}", err),
    };

    init_db(&db_path);

    tauri::Builder::default()
        .setup(|app| {
            // https://github.com/tauri-apps/tauri/issues/1213#issuecomment-1700917797
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .manage(DbPath(Mutex::new(db_path)))
        .invoke_handler(tauri::generate_handler![
            get_parts,
            get_categories,
            part_add,
            part_new,
            part_del,
            new_category,
            del_category,
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
    let mut p = GET!(db);
    *p = String::from(new_path);
}

fn init_db(path: &str) {
    let db = elebox_core::JammDatebase::new(path);
    db.init();
}
