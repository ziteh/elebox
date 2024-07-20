// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs;
use elebox_core::{
    Category, CustomField, Manufacturer, Package, PackageType, Part, Supplier, TreeNode,
};
use std::{path::Path, sync::Mutex, task::ready};
use tauri::Manager;

macro_rules! GET {
    ($db:expr) => {
        $db.0.lock().unwrap()
    };
}

struct DbPath(Mutex<String>);

#[tauri::command(rename_all = "snake_case")]
fn get_part(path: tauri::State<DbPath>, name: &str) -> Option<Part> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&db);
    let part = mgr.get(name).ok()?;
    Some(part)
}

#[tauri::command(rename_all = "snake_case")]
fn get_parts(path: tauri::State<DbPath>) -> Vec<Part> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&db);
    mgr.list()
}

#[tauri::command(rename_all = "snake_case")]
fn add_part(path: tauri::State<DbPath>, item: Part) -> Result<(), String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&db);
    mgr.add(&item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_part(path: tauri::State<DbPath>, ori_name: &str, new_item: Part) -> Result<(), String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&db);
    mgr.update(ori_name, &new_item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_part(path: tauri::State<DbPath>, name: &str) -> Result<String, String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&db);
    mgr.delete(name).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn increment_part(path: tauri::State<DbPath>, name: &str, increment: i16) -> Result<(), String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&db);
    mgr.update_part_quantity(name, increment)
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn get_category(path: tauri::State<DbPath>, name: &str) -> Option<Category> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&db);
    let cat = mgr.get(name).ok()?;
    Some(cat)
}

#[tauri::command(rename_all = "snake_case")]
fn get_categories(path: tauri::State<DbPath>) -> Vec<Category> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&db);
    mgr.list()
}

#[tauri::command(rename_all = "snake_case")]
fn add_category(path: tauri::State<DbPath>, item: Category) -> Result<(), String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&db);
    mgr.add(&item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_category(
    path: tauri::State<DbPath>,
    ori_name: &str,
    new_item: Category,
) -> Result<(), String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&db);
    mgr.update(ori_name, &new_item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_category(path: tauri::State<DbPath>, name: &str) -> Result<String, String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&db);
    mgr.delete(name).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn get_tree(path: tauri::State<DbPath>) -> Vec<TreeNode> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&db);
    mgr.get_tree()
}

#[tauri::command(rename_all = "snake_case")]
fn get_package(path: tauri::State<DbPath>, name: &str) -> Option<Package> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&db);
    let pkg = mgr.get(name).ok()?;
    Some(pkg)
}

#[tauri::command(rename_all = "snake_case")]
fn get_packages(path: tauri::State<DbPath>) -> Vec<Package> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&db);
    mgr.list()
}

#[tauri::command(rename_all = "snake_case")]
fn add_package(path: tauri::State<DbPath>, item: Package) -> Result<(), String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&db);
    mgr.add(&item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_package(
    path: tauri::State<DbPath>,
    ori_name: &str,
    new_item: Package,
) -> Result<(), String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&db);
    mgr.update(ori_name, &new_item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_package(path: tauri::State<DbPath>, name: &str) -> Result<(), String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&db);
    mgr.delete(name).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn get_mfr(path: tauri::State<DbPath>, name: &str) -> Option<Manufacturer> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&db);
    let mfr = mgr.get(name).ok()?;
    Some(mfr)
}

#[tauri::command(rename_all = "snake_case")]
fn get_mfrs(path: tauri::State<DbPath>) -> Vec<Manufacturer> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&db);
    mgr.list()
}

#[tauri::command(rename_all = "snake_case")]
fn add_mfr(path: tauri::State<DbPath>, item: Manufacturer) -> Result<(), String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&db);
    mgr.add(&item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_mfr(
    path: tauri::State<DbPath>,
    ori_name: &str,
    new_item: Manufacturer,
) -> Result<(), String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&db);
    mgr.update(ori_name, &new_item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_mfr(path: tauri::State<DbPath>, name: &str) -> Result<String, String> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&db);
    mgr.delete(name).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn get_db_path(path: tauri::State<DbPath>) -> String {
    GET!(path).to_string()
}

#[tauri::command(rename_all = "snake_case")]
fn get_default_db_path() -> String {
    if let Ok(path) = get_default_path() {
        return path;
    }
    return "".to_string();
}

#[tauri::command(rename_all = "snake_case")]
fn set_db_path(path: tauri::State<DbPath>, new_path: &str) {
    update_db_path(path, new_path);
    init_db(new_path);
}

#[tauri::command(rename_all = "snake_case")]
fn export_csv(path: tauri::State<DbPath>, csv_path: &str) {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    elebox_core::export(&db, csv_path);
}

#[tauri::command(rename_all = "snake_case")]
fn import_csv(csv_path: &str) -> Result<(), String> {
    elebox_core::import(csv_path)
}

fn main() {
    let mut db_path = match get_default_path() {
        Ok(path) => path,
        Err(err) => panic!("{}", err),
    };

    if Path::new(&db_path).exists() {
        init_db(&db_path);
    } else {
        db_path = "".to_string();
    }

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                // https://github.com/tauri-apps/tauri/issues/1213#issuecomment-1700917797
                app.get_window("main").unwrap().open_devtools();
                Ok(())
            }
            #[cfg(not(debug_assertions))]
            {
                Ok(())
            }
        })
        .manage(DbPath(Mutex::new(db_path)))
        .invoke_handler(tauri::generate_handler![
            get_part,
            get_parts,
            add_part,
            update_part,
            del_part,
            increment_part,
            get_category,
            get_categories,
            add_category,
            update_category,
            del_category,
            get_tree,
            get_package,
            get_packages,
            add_package,
            update_package,
            del_package,
            get_mfr,
            get_mfrs,
            add_mfr,
            update_mfr,
            del_mfr,
            get_db_path,
            get_default_db_path,
            set_db_path,
            export_csv,
            import_csv,
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
    elebox_core::create_default_db(path);
    // let db = elebox_core::JammDatabase::new(path);
    // db.init();
}
