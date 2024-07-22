// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

use dirs;
use elebox_core::{
    Category, CustomField, Database, Manager, Manufacturer, Package, PackageType, Part, Supplier,
    TreeNode,
};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
    task::ready,
};
use tauri::Manager as TauriManager;

macro_rules! GET {
    ($db:expr) => {
        $db.0.lock().unwrap()
    };
}

struct UserDir(Mutex<String>);
struct DbPath(Mutex<String>);
struct Language(Mutex<String>);

#[tauri::command(rename_all = "snake_case")]
fn get_part(path: tauri::State<DbPath>, name: &str) -> Option<Part> {
    let p = GET!(path);
    // let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&p);
    let part = mgr.get(name).ok()?;
    Some(part)
}

#[tauri::command(rename_all = "snake_case")]
fn get_parts(path: tauri::State<DbPath>) -> Vec<Part> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&p);
    mgr.list().unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn add_part(path: tauri::State<DbPath>, item: Part) -> Result<(), String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&p);
    mgr.add(&item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_part(path: tauri::State<DbPath>, ori_name: &str, new_item: Part) -> Result<(), String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&p);
    mgr.update(ori_name, &new_item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_part(path: tauri::State<DbPath>, name: &str) -> Result<String, String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&p);
    mgr.delete(name);
    Ok("ok".to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn increment_part(path: tauri::State<DbPath>, name: &str, increment: i16) -> Result<(), String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&p);
    if let Err(err) = mgr.update_part_quantity(name, increment) {
        return Err(err.to_string());
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn get_category(path: tauri::State<DbPath>, name: &str) -> Option<Category> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&p);
    let cat = mgr.get(name).ok()?;
    Some(cat)
}

#[tauri::command(rename_all = "snake_case")]
fn get_categories(path: tauri::State<DbPath>) -> Vec<Category> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&p);
    mgr.list().unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn add_category(path: tauri::State<DbPath>, item: Category) -> Result<(), String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&p);
    mgr.add(&item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_category(
    path: tauri::State<DbPath>,
    ori_name: &str,
    new_item: Category,
) -> Result<(), String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&p);
    mgr.update(ori_name, &new_item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_category(path: tauri::State<DbPath>, name: &str) -> Result<String, String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&p);
    mgr.delete(name);
    Ok("ok".to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn get_tree(path: tauri::State<DbPath>) -> Vec<TreeNode> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&p);
    mgr.get_tree().unwrap() // TODO
}

#[tauri::command(rename_all = "snake_case")]
fn get_package(path: tauri::State<DbPath>, name: &str) -> Option<Package> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&p);
    let pkg = mgr.get(name).ok()?;
    Some(pkg)
}

#[tauri::command(rename_all = "snake_case")]
fn get_packages(path: tauri::State<DbPath>) -> Vec<Package> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&p);
    mgr.list().unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn add_package(path: tauri::State<DbPath>, item: Package) -> Result<(), String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&p);
    mgr.add(&item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_package(
    path: tauri::State<DbPath>,
    ori_name: &str,
    new_item: Package,
) -> Result<(), String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&p);
    mgr.update(ori_name, &new_item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_package(path: tauri::State<DbPath>, name: &str) -> Result<(), String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&p);
    mgr.delete(name);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn get_mfr(path: tauri::State<DbPath>, name: &str) -> Option<Manufacturer> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&p);
    let mfr = mgr.get(name).ok()?;
    Some(mfr)
}

#[tauri::command(rename_all = "snake_case")]
fn get_mfrs(path: tauri::State<DbPath>) -> Vec<Manufacturer> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&p);
    mgr.list().unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn add_mfr(path: tauri::State<DbPath>, item: Manufacturer) -> Result<(), String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&p);
    mgr.add(&item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn update_mfr(
    path: tauri::State<DbPath>,
    ori_name: &str,
    new_item: Manufacturer,
) -> Result<(), String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&p);
    mgr.update(ori_name, &new_item).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn del_mfr(path: tauri::State<DbPath>, name: &str) -> Result<String, String> {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&p);
    let _ = mgr.delete(name);
    Ok("ok".to_string())
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
        return Err("not found".to_string());
    }

    if !check_db(&new_path) {
        return Err("not database".to_string());
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
    let db_path = GET!(db);
    PathBuf::from(db_path.to_string()).exists()
}

#[tauri::command(rename_all = "snake_case")]
fn get_language(lang: tauri::State<Language>) -> String {
    GET!(lang).to_string()
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
fn export_csv(path: tauri::State<DbPath>, csv_path: &str) {
    let p = GET!(path);
    //  let db = elebox_core::JammDatabase::new(&p);
    elebox_core::export(&p, csv_path);
}

#[tauri::command(rename_all = "snake_case")]
fn import_csv(csv_path: &str) -> Result<(), String> {
    elebox_core::import(csv_path)
}

fn main() {
    let user_dir = get_user_dir().unwrap();

    config::create_config(&user_dir);
    let mut config = config::load_config(&user_dir).unwrap();

    if config.database.is_none() {
        config.database = match get_default_path() {
            Ok(path) => Some(path),
            Err(err) => panic!("{}", err), // TODO
        };
    }
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
    let mut p = GET!(db);
    *p = String::from(new_path);
}

fn set_config_language(state: tauri::State<Language>, language: &str) {
    let mut mutex = GET!(state);
    *mutex = String::from(language);
}

fn set_user_dir(state: tauri::State<UserDir>, dir: &str) {
    let mut mutex = GET!(state);
    *mutex = String::from(dir);
}

fn init_db(path: &str) {
    elebox_core::create_default_db(path);
    // let db = elebox_core::JammDatabase::new(path);
    // db.init();
}

fn set_config(dir: tauri::State<UserDir>, lang: tauri::State<Language>, db: tauri::State<DbPath>) {
    let dir = GET!(dir).to_string();
    let config = config::Config {
        language: Some(GET!(lang).to_string()),
        database: Some(GET!(db).to_string()),
    };
    config::save_config(&dir, &config);
}

fn check_db(path: &str) -> bool {
    return elebox_core::check_db(&path).is_ok();
    // if path.is_none() {
    //     let db_path = GET!(db);
    //     return elebox_core::check_db(&db_path).is_ok();
    // } else {
    //     return elebox_core::check_db(&path.unwrap()).is_ok();
    // }
}
