use crate::{
    CategoryManager, JammDatabase, Manager, ManufacturerManager, PackageManager, PartManager,
    Transferable,
};

pub fn export(db: &str, path: &str) {
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
    let db = format!("{}{}", path, "import_elebox.db");

    let filename_mfr = format!("{}{}", path, "elebox_export_mfrs.yaml");
    let mfr_mgr = ManufacturerManager::new(&db);
    mfr_mgr.init();
    mfr_mgr.import(&filename_mfr);

    let filename_pkg = format!("{}{}", path, "elebox_export_packages.yaml");
    let mfr_pkg = PackageManager::new(&db);
    mfr_pkg.init();
    mfr_pkg.import(&filename_pkg);

    let filename_cat = format!("{}{}", path, "elebox_export_categories.yaml");
    let mfr_cat = CategoryManager::new(&db);
    mfr_cat.init();
    mfr_cat.import(&filename_cat);

    let filename_part = format!("{}{}", path, "elebox_export_parts.yaml");
    let mfr_part = PartManager::new(&db);
    mfr_part.init();
    mfr_part.import(&filename_part);

    Ok(())
}
