use crate::{
    CategoryManager, Database, JammDatabase, ManufacturerManager, PackageManager, PartManager,
};
use std::path::PathBuf;

pub fn export(db: &dyn Database, path: &str) {
    let filename_part = format!("{}{}", path, "elebox_export_parts.yaml");
    let _ = PartManager::new(db).export(&filename_part);

    let filename_cat = format!("{}{}", path, "elebox_export_categories.yaml");
    let _ = CategoryManager::new(db).export(&filename_cat);

    let filename_pkg = format!("{}{}", path, "elebox_export_packages.yaml");
    let _ = PackageManager::new(db).export(&filename_pkg);

    let filename_mfr = format!("{}{}", path, "elebox_export_mfrs.yaml");
    let _ = ManufacturerManager::new(db).export(&filename_mfr);
}

pub fn import(path: &str) -> Result<(), String> {
    let db_path = format!("{}{}", path, "import_elebox.db");
    let db = JammDatabase::new(&db_path);
    db.init();

    let filename_mfr = format!("{}{}", path, "elebox_export_mfrs.yaml");
    let res = ManufacturerManager::new(&db).import(&filename_mfr);
    if res.is_err() {
        return Err("Part".to_string());
    }

    let filename_pkg = format!("{}{}", path, "elebox_export_packages.yaml");
    let res = PackageManager::new(&db).import(&filename_pkg);
    if res.is_err() {
        return Err("Part".to_string());
    }

    let filename_cat = format!("{}{}", path, "elebox_export_categories.yaml");
    let res = CategoryManager::new(&db).import(&filename_cat);
    if res.is_err() {
        return Err("Part".to_string());
    }

    let filename_part = format!("{}{}", path, "elebox_export_parts.yaml");
    let res = PartManager::new(&db).import(&filename_part);
    if res.is_err() {
        return Err("Part".to_string());
    }

    Ok(())
}
