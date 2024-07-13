mod category;
mod csv;
mod db;
mod default_db;
mod errors;
mod manufacturer;
mod package;
mod part;

pub use category::*;
pub use db::Database;
pub use db::JammDatabase;
pub use errors::*;
pub use manufacturer::*;
pub use package::*;
pub use part::*;

pub use default_db::create_default_db;

pub fn export_csv(db: &dyn Database, path: &str) {
    let filename_part = format!("{}{}", path, "elebox_export_parts.tsv");
    let _ = PartManager::new(db).export_csv(&filename_part);

    let filename_cat = format!("{}{}", path, "elebox_export_categories.tsv");
    let _ = CategoryManager::new(db).export_csv(&filename_cat);

    let filename_pkg = format!("{}{}", path, "elebox_export_packages.tsv");
    let _ = PackageManager::new(db).export_csv(&filename_pkg);

    let filename_mfr = format!("{}{}", path, "elebox_export_mfrs.tsv");
    let _ = ManufacturerManager::new(db).export_csv(&filename_mfr);
}

pub fn import_csv(path: &str) -> Result<(), String> {
    let db_path = format!("{}{}", path, "import_elebox.db");
    let db = JammDatabase::new(&db_path);
    db.init();

    let filename_mfr = format!("{}{}", path, "elebox_export_mfrs.tsv");
    let res = ManufacturerManager::new(&db).import_csv(&filename_mfr);
    if res.is_err() {
        return Err("Part".to_string());
    }

    let filename_pkg = format!("{}{}", path, "elebox_export_packages.tsv");
    let res = PackageManager::new(&db).import_csv(&filename_pkg);
    if res.is_err() {
        return Err("Part".to_string());
    }

    let filename_cat = format!("{}{}", path, "elebox_export_categories.tsv");
    let res = CategoryManager::new(&db).import_csv(&filename_cat);
    if res.is_err() {
        return Err("Part".to_string());
    }

    let filename_part = format!("{}{}", path, "elebox_export_parts.tsv");
    let res = PartManager::new(&db).import_csv(&filename_part);
    if res.is_err() {
        return Err("Part".to_string());
    }

    Ok(())
}
