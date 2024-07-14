// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs;
use elebox_core::{Category, Database, Manufacturer, Package, PackageType, Part, TreeNode};
use std::sync::Mutex;
use tauri::Manager;

macro_rules! GET {
    ($db:expr) => {
        $db.0.lock().unwrap()
    };
}

struct DbPath(Mutex<String>);

#[tauri::command]
fn get_packages(path: tauri::State<DbPath>) -> Vec<Package> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&db);
    mgr.list()
}

#[tauri::command]
fn new_package(path: tauri::State<DbPath>, name: &str, pkg_type: &str, alias: &str) {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&db);

    let pkg = Package {
        name: name.to_string(),
        pkg_type: match pkg_type.to_uppercase().as_str() {
            "SMT" => PackageType::Smt,
            "THT" => PackageType::Tht,
            _ => PackageType::Others,
        },
        alias: match alias {
            "" => None,
            _ => Some(alias.to_string()),
        },
    };
    let _ = mgr.add(&pkg);
}

#[tauri::command]
fn del_package(path: tauri::State<DbPath>, name: &str) {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PackageManager::new(&db);
    let _ = mgr.delete(name);
}

#[tauri::command]
fn get_mfrs(path: tauri::State<DbPath>) -> Vec<Manufacturer> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&db);
    mgr.list()
}

#[tauri::command]
fn new_mfr(path: tauri::State<DbPath>, name: &str, alias: &str, url: &str) {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&db);

    let mfr = Manufacturer {
        name: name.to_string(),
        alias: match alias {
            "" => None,
            _ => Some(alias.to_string()),
        },
        url: match url {
            "" => None,
            _ => Some(url.to_string()),
        },
    };
    let _ = mgr.add(&mfr);
}

#[tauri::command]
fn del_mfr(path: tauri::State<DbPath>, name: &str) {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::ManufacturerManager::new(&db);
    let _ = mgr.delete(name);
}

#[tauri::command]
fn get_part(path: tauri::State<DbPath>, part: &str) -> Option<Part> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&db);
    let part = mgr.get(part);
    if part.is_err() {
        return None;
    }
    return Some(part.unwrap());
}

#[tauri::command]
fn get_parts(path: tauri::State<DbPath>) -> Vec<Part> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&db);
    mgr.list()
}

#[tauri::command]
fn get_tree(path: tauri::State<DbPath>) -> Vec<TreeNode> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&db);
    mgr.get_tree()
}

#[tauri::command]
fn part_del(path: tauri::State<DbPath>, part: &str) {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&db);
    let _ = mgr.delete(part);
}

#[tauri::command]
fn part_add(path: tauri::State<DbPath>, part: &str, qty: i16) {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&db);
    let _ = mgr.update_part_quantity(part, qty);
}

#[tauri::command]
fn new_category(path: tauri::State<DbPath>, name: &str, parent: &str) {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&db);

    let cat = Category {
        name: name.to_string(),
        parent: match parent {
            "" => None,
            _ => Some(parent.to_string()),
        },
        alias: None, // TODO
    };
    let _ = mgr.add(&cat);
}

#[tauri::command]
fn del_category(path: tauri::State<DbPath>, name: &str) -> String {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::CategoryManager::new(&db);

    let res = mgr.delete(name);
    match res {
        Err(e) => return e.to_string(),
        Ok(s) => return format!("OK {s}"),
    }
}

#[tauri::command]
fn new_part(
    path: tauri::State<DbPath>,
    name: &str,
    qty: u16,
    category: &str,
    package: &str,
    package_detail: &str,
    mfr: &str,
    alias: &str,
    description: &str,
    cost: f32,
    location: &str,
    mfr_no: &str,
    mouser_no: &str,
    digikey_no: &str,
    datasheet_link: &str,
    product_link: &str,
    image_link: &str,
    suppliers: &str,
) {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    let mgr = elebox_core::PartManager::new(&db);

    let mut part = Part::new(name, category, qty);

    part.package = Option::from(package.to_string()).filter(|s| !s.is_empty());
    part.package_detail = Option::from(package_detail.to_string()).filter(|s| !s.is_empty());
    part.mfr = Option::from(mfr.to_string()).filter(|s| !s.is_empty());
    part.alias = Option::from(alias.to_string()).filter(|s| !s.is_empty());
    part.description = Option::from(description.to_string()).filter(|s| !s.is_empty());
    part.location = Option::from(location.to_string()).filter(|s| !s.is_empty());
    part.mfr_no = Option::from(mfr_no.to_string()).filter(|s| !s.is_empty());
    part.mouser_no = Option::from(mouser_no.to_string()).filter(|s| !s.is_empty());
    part.digikey_no = Option::from(digikey_no.to_string()).filter(|s| !s.is_empty());
    part.datasheet_link = Option::from(datasheet_link.to_string()).filter(|s| !s.is_empty());
    part.product_link = Option::from(product_link.to_string()).filter(|s| !s.is_empty());
    part.image_link = Option::from(image_link.to_string()).filter(|s| !s.is_empty());
    part.suppliers = Option::from(suppliers.to_string()).filter(|s| !s.is_empty());

    if cost < 0.0 {
        part.cost = None;
    } else {
        part.cost = Some(cost);
    }

    let _ = mgr.add(&part);
}

#[tauri::command]
fn get_categories(path: tauri::State<DbPath>) -> Vec<Category> {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
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

#[tauri::command]
fn export_csv(path: tauri::State<DbPath>, csv_path: &str) {
    let p = GET!(path);
    let db = elebox_core::JammDatabase::new(&p);
    elebox_core::export_csv(&db, csv_path);
}

#[tauri::command]
fn import_csv(path: tauri::State<DbPath>, csv_path: &str) {
    let _ = elebox_core::import_csv(csv_path);
}

fn main() {
    let db_path = match get_default_path() {
        Ok(path) => path,
        Err(err) => panic!("{}", err),
    };

    init_db(&db_path);

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
            get_parts,
            get_part,
            get_categories,
            part_add,
            new_part,
            part_del,
            new_category,
            del_category,
            get_db_path,
            set_db_path,
            get_packages,
            new_package,
            del_package,
            get_mfrs,
            new_mfr,
            del_mfr,
            export_csv,
            import_csv,
            get_tree,
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
