// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

use dirs::{self, document_dir};
use elebox_core::{
    Category, CustomField, Database, Handler, JammDatabase, Manager, Manufacturer, Package,
    PackageType, Part, Supplier, TreeNode,
};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
    task::ready,
};
use tauri::Manager as TauriManager;

macro_rules! lock {
    ($db:expr) => {
        $db.0.lock().unwrap()
    };
}

struct EleboxManager(Mutex<Manager>);
struct UserDir(Mutex<String>);
struct DbPath(Mutex<String>);
struct Language(Mutex<String>);

#[tauri::command(rename_all = "snake_case")]
fn get_part(manager: tauri::State<EleboxManager>, name: &str) -> Option<Part> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.part();
    let part = hdr.get(name).ok()?;
    Some(part)
}

#[tauri::command(rename_all = "snake_case")]
fn get_parts(manager: tauri::State<EleboxManager>) -> Vec<Part> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.part();
    hdr.list().unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn add_part(manager: tauri::State<EleboxManager>, item: Part) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.part();
    hdr.add(&item).map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_part(
    manager: tauri::State<EleboxManager>,
    ori_name: &str,
    new_item: Part,
) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.part();
    hdr.update(ori_name, &new_item)
        .map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_part(manager: tauri::State<EleboxManager>, name: &str) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.part();
    hdr.delete(name).map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn increment_part(
    manager: tauri::State<EleboxManager>,
    name: &str,
    increment: i16,
) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.part();
    if let Err(err) = hdr.update_part_quantity(name, increment) {
        return Err(err.to_string());
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn get_category(manager: tauri::State<EleboxManager>, name: &str) -> Option<Category> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.category();
    let part = hdr.get(name).ok()?;
    Some(part)
}

#[tauri::command(rename_all = "snake_case")]
fn get_categories(manager: tauri::State<EleboxManager>) -> Vec<Category> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.category();
    hdr.list().unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn add_category(manager: tauri::State<EleboxManager>, item: Category) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.category();
    hdr.add(&item).map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_category(
    manager: tauri::State<EleboxManager>,
    ori_name: &str,
    new_item: Category,
) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.category();
    hdr.update(ori_name, &new_item)
        .map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_category(manager: tauri::State<EleboxManager>, name: &str) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.category();
    hdr.delete(name).map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn get_tree(manager: tauri::State<EleboxManager>) -> Vec<TreeNode> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.category();
    hdr.get_tree().unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn get_package(manager: tauri::State<EleboxManager>, name: &str) -> Option<Package> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.package();
    let part = hdr.get(name).ok()?;
    Some(part)
}

#[tauri::command(rename_all = "snake_case")]
fn get_packages(manager: tauri::State<EleboxManager>) -> Vec<Package> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.package();
    hdr.list().unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn add_package(manager: tauri::State<EleboxManager>, item: Package) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.package();
    hdr.add(&item).map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_package(
    manager: tauri::State<EleboxManager>,
    ori_name: &str,
    new_item: Package,
) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.package();
    hdr.update(ori_name, &new_item)
        .map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_package(manager: tauri::State<EleboxManager>, name: &str) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.package();
    hdr.delete(name).map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn get_mfr(manager: tauri::State<EleboxManager>, name: &str) -> Option<Manufacturer> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.manufacturer();
    let part = hdr.get(name).ok()?;
    Some(part)
}

#[tauri::command(rename_all = "snake_case")]
fn get_mfrs(manager: tauri::State<EleboxManager>) -> Vec<Manufacturer> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.manufacturer();
    hdr.list().unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn add_mfr(manager: tauri::State<EleboxManager>, item: Manufacturer) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.manufacturer();
    hdr.add(&item).map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_mfr(
    manager: tauri::State<EleboxManager>,
    ori_name: &str,
    new_item: Manufacturer,
) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.manufacturer();
    hdr.update(ori_name, &new_item)
        .map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_mfr(manager: tauri::State<EleboxManager>, name: &str) -> Result<(), String> {
    let mgr_lock = lock!(manager);
    let hdr = mgr_lock.manufacturer();
    hdr.delete(name).map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn get_db_path(path: tauri::State<DbPath>) -> String {
    lock!(path).to_string()
}

#[tauri::command(rename_all = "snake_case")]
fn get_default_db_path() -> String {
    if let Ok(path) = get_default_database_path() {
        return path;
    }
    return "".to_string();
}

#[tauri::command(rename_all = "snake_case")]
fn get_assets_path() -> Result<String, String> {
    if let Some(mut dir) = dirs::data_local_dir() {
        dir.push("elebox");
        dir.push("assets");
        dir.push(""); // Add ending '/'

        if let Err(err) = std::fs::create_dir_all(&dir) {
            return Err(format!("Unable to creating directory. {}", err));
        }
        return Ok(dir.to_string_lossy().into_owned());
    } else {
        return Err("Unable to determine user's local data directory.".to_string());
    }
}

#[tauri::command(rename_all = "snake_case")]
fn set_db_path(
    dir: tauri::State<UserDir>,
    lang: tauri::State<Language>,
    path: tauri::State<DbPath>,
    new_path: &str,
) -> Result<(), String> {
    let p = PathBuf::from(new_path);
    if !p.exists() {
        return Err("File does not exist".to_string());
    }

    if !check_db(&new_path) {
        return Err("This file is not an database".to_string());
    }

    update_db_path(path.clone(), new_path);
    set_config(dir, lang, path);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn create_db(
    dir: tauri::State<UserDir>,
    lang: tauri::State<Language>,
    path: tauri::State<DbPath>,
    new_path: &str,
    empty: bool,
) -> Result<(), String> {
    update_db_path(path.clone(), new_path);
    init_db(new_path);
    set_config(dir, lang, path);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn is_db_exists(db: tauri::State<DbPath>) -> bool {
    let db_path = lock!(db);
    PathBuf::from(db_path.to_string()).exists()
}

#[tauri::command(rename_all = "snake_case")]
fn get_language(lang: tauri::State<Language>) -> String {
    lock!(lang).to_string()
}

#[tauri::command(rename_all = "snake_case")]
fn set_language(
    dir: tauri::State<UserDir>,
    lang: tauri::State<Language>,
    path: tauri::State<DbPath>,
    new_lang: &str,
) {
    set_config_language(lang.clone(), new_lang);
    set_config(dir, lang, path);
}

#[tauri::command(rename_all = "snake_case")]
fn export_csv(manager: tauri::State<EleboxManager>, csv_path: &str) {
    // let p = GET!(path);

    // elebox_core::export(&p, csv_path);
    todo!()
}

#[tauri::command(rename_all = "snake_case")]
fn import_csv(csv_path: &str) -> Result<(), String> {
    // elebox_core::import(csv_path)
    todo!()
}

fn main() {
    let user_dir = get_user_dir().unwrap();

    config::create_config(&user_dir);
    let mut config = config::load_config(&user_dir).unwrap();

    if config.database.is_none() {
        config.database = match get_default_database_path() {
            Ok(path) => Some(path),
            Err(err) => panic!("{}", err), // TODO
        };
    }

    let part_db = Box::new(JammDatabase::new(&config.database.clone().unwrap()));
    let pkg_db = Box::new(JammDatabase::new(&config.database.clone().unwrap()));
    let cat_db = Box::new(JammDatabase::new(&config.database.clone().unwrap()));
    let mfr_db = Box::new(JammDatabase::new(&config.database.clone().unwrap()));
    let manager = elebox_core::Manager::new(part_db, pkg_db, cat_db, mfr_db);
    // init_db(&config.database.clone().unwrap());

    if config.language.is_none() {
        config.language = Some("en".to_string());
    }

    config::save_config(&user_dir, &config);

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
        .manage(UserDir(Mutex::new(user_dir)))
        .manage(DbPath(Mutex::new(config.database.unwrap())))
        .manage(Language(Mutex::new(config.language.unwrap())))
        .manage(EleboxManager(Mutex::new(manager)))
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
            get_assets_path,
            get_default_db_path,
            set_db_path,
            is_db_exists,
            set_language,
            get_language,
            create_db,
            export_csv,
            import_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_default_database_path() -> Result<String, String> {
    let user_dir = get_user_dir()?;
    let db_path = PathBuf::from(user_dir).join("elebox.db");
    Ok(db_path.to_string_lossy().into_owned())
}

fn get_user_dir() -> Result<String, String> {
    if let Some(mut dir) = dirs::data_local_dir() {
        dir.push("elebox");
        dir.push(""); // Keep ending '/'

        if let Err(err) = std::fs::create_dir_all(&dir) {
            return Err(format!("Unable to creating directory. {}", err));
        }

        return Ok(dir.to_string_lossy().into_owned());
    } else {
        return Err("Unable to determine user's local data directory.".to_string());
    }
}

fn update_db_path(db: tauri::State<DbPath>, new_path: &str) {
    let mut p = lock!(db);
    *p = String::from(new_path);
}

fn set_config_language(state: tauri::State<Language>, language: &str) {
    let mut mutex = lock!(state);
    *mutex = String::from(language);
}

fn set_user_dir(state: tauri::State<UserDir>, dir: &str) {
    let mut mutex = lock!(state);
    *mutex = String::from(dir);
}

fn init_db(path: &str) {
    elebox_core::create_default_db(path);
    // let db = elebox_core::JammDatabase::new(path);
    // db.init();
}

fn set_config(dir: tauri::State<UserDir>, lang: tauri::State<Language>, db: tauri::State<DbPath>) {
    let dir = lock!(dir).to_string();
    let config = config::Config {
        language: Some(lock!(lang).to_string()),
        database: Some(lock!(db).to_string()),
    };
    config::save_config(&dir, &config);
}

fn check_db(path: &str) -> bool {
    true
    // return elebox_core::check_db(&path).is_ok();
    // if path.is_none() {
    //     let db_path = GET!(db);
    //     return elebox_core::check_db(&db_path).is_ok();
    // } else {
    //     return elebox_core::check_db(&path.unwrap()).is_ok();
    // }
}
