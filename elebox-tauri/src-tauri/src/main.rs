// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

use config::{save_config, Config};
use dirs::{self};
use elebox_core::{
    Category, Handler, JammDatabase, Manager, Manufacturer, Package, Part, TreeNode,
};
use std::{path::PathBuf, sync::Mutex};
use tauri::Manager as TauriManager;

macro_rules! lock {
    ($db:expr) => {
        $db.0.lock().unwrap()
    };
}

struct EleboxManager(Mutex<Manager>);
struct EleboxConfig(Mutex<Config>);
struct EleboxUserDir(Mutex<PathBuf>);

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
fn get_db_path(config: tauri::State<EleboxConfig>) -> String {
    let cfg = lock!(config);
    cfg.database.clone().unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn get_default_db_path() -> Result<String, String> {
    if let Ok(path) = get_default_database_path() {
        return Ok(path.to_string_lossy().into_owned());
    }
    return Err("".to_string()); // TODO
}

#[tauri::command(rename_all = "snake_case")]
fn get_assets_path() -> Result<String, String> {
    if let Ok(dir) = get_assets_dir() {
        return Ok(dir.to_string_lossy().into_owned());
    }
    return Err("".to_string()); // TODO
}

#[tauri::command(rename_all = "snake_case")]
fn set_db_path(
    config: tauri::State<EleboxConfig>,
    user_dir: tauri::State<EleboxUserDir>,
    new_path: &str,
) -> Result<(), String> {
    let path = PathBuf::from(new_path);
    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    if !check_db(&new_path) {
        return Err("This file is not an database".to_string());
    }

    let mut cfg = lock!(config);
    cfg.database = Some(path.to_string_lossy().into_owned());

    let dir = lock!(user_dir);
    let _ = save_config(&dir, &cfg);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn create_db(
    config: tauri::State<EleboxConfig>,
    user_dir: tauri::State<EleboxUserDir>,
    new_path: &str,
    _empty: bool,
) -> Result<(), String> {
    let path = PathBuf::from(new_path);
    let mut cfg = lock!(config);
    cfg.database = Some(path.to_string_lossy().into_owned());

    init_db(new_path);

    let dir = lock!(user_dir);
    let _ = save_config(&dir, &cfg);

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn is_db_exists(config: tauri::State<EleboxConfig>) -> bool {
    let cfg = lock!(config);
    if let Some(path) = &cfg.database {
        return PathBuf::from(path).exists();
    }
    false
}

#[tauri::command(rename_all = "snake_case")]
fn get_language(config: tauri::State<EleboxConfig>) -> String {
    let cfg = lock!(config);
    if let Some(lang) = &cfg.language {
        return lang.to_string();
    }
    String::from("en") // TODO
}

#[tauri::command(rename_all = "snake_case")]
fn set_language(
    config: tauri::State<EleboxConfig>,
    user_dir: tauri::State<EleboxUserDir>,
    new_lang: &str,
) {
    let mut cfg = lock!(config);
    cfg.language = Some(String::from(new_lang));

    let dir = lock!(user_dir);
    let _ = save_config(&dir, &cfg);
}

#[tauri::command(rename_all = "snake_case")]
fn export_csv(manager: tauri::State<EleboxManager>, csv_path: &str) {
    let mgr_lock = lock!(manager);
    let _ = mgr_lock.export(&PathBuf::from(csv_path));
}

#[tauri::command(rename_all = "snake_case")]
fn import_csv(_manager: tauri::State<EleboxManager>, csv_path: &str) -> Result<(), String> {
    // TODO
    let path = PathBuf::from(csv_path);
    let db_path = path.join("import_exebox.db");
    let part_db = Box::new(JammDatabase::new(&db_path.to_string_lossy().into_owned()));
    let pkg_db = Box::new(JammDatabase::new(&db_path.to_string_lossy().into_owned()));
    let cat_db = Box::new(JammDatabase::new(&db_path.to_string_lossy().into_owned()));
    let mfr_db = Box::new(JammDatabase::new(&db_path.to_string_lossy().into_owned()));
    let _ = Manager::from(part_db, pkg_db, cat_db, mfr_db, &path);
    Ok(())
}

fn main() {
    let user_dir = get_user_dir().unwrap();

    let _ = config::create_config(&user_dir);
    let mut config = config::load_config(&user_dir).unwrap();

    if config.database.is_none() {
        config.database = match get_default_database_path() {
            Ok(path) => Some(path.to_string_lossy().into_owned()),
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

    let _ = config::save_config(&user_dir, &config);

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
        .manage(EleboxManager(Mutex::new(manager)))
        .manage(EleboxConfig(Mutex::new(config)))
        .manage(EleboxUserDir(Mutex::new(user_dir)))
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

fn get_user_dir() -> Result<PathBuf, String> {
    if let Some(mut dir) = dirs::data_local_dir() {
        dir.push("elebox");
        dir.push(""); // Keep ending '/'

        if let Err(err) = std::fs::create_dir_all(&dir) {
            return Err(format!("Unable to creating directory. {}", err));
        }

        return Ok(dir);
    } else {
        return Err("Unable to determine user's local data directory.".to_string());
    }
}

fn get_default_database_path() -> Result<PathBuf, String> {
    let user_dir = get_user_dir()?;
    let path = user_dir.join("elebox.db");
    Ok(path)
}

fn get_assets_dir() -> Result<PathBuf, String> {
    let user_dir = get_user_dir()?;
    let dir = user_dir.join("assets").join(""); // Keep ending '/'
    Ok(dir)
}

fn init_db(path: &str) {
    elebox_core::create_default_db(path);
}

// TODO
fn check_db(_path: &str) -> bool {
    true

    // return elebox_core::check_db(&path).is_ok();
    // if path.is_none() {
    //     let db_path = GET!(db);
    //     return elebox_core::check_db(&db_path).is_ok();
    // } else {
    //     return elebox_core::check_db(&path.unwrap()).is_ok();
    // }
}
