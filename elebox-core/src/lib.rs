mod category;
mod csv;
mod db;
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

pub fn export_csv(db: &dyn Database, path: &str) {
    let filename_part = format!("{}{}", path, "elebox_export_parts.tsv");
    let _ = PartManager::new(db).save_csv(&filename_part);

    let filename_cat = format!("{}{}", path, "elebox_export_categories.tsv");
    let _ = CategoryManager::new(db).save_csv(&filename_cat);

    let filename_pkg = format!("{}{}", path, "elebox_export_packages.tsv");
    let _ = PackageManager::new(db).save_csv(&filename_pkg);

    let filename_mfr = format!("{}{}", path, "elebox_export_manufacturers.tsv");
    let _ = ManufacturerManager::new(db).save_csv(&filename_mfr);
}
