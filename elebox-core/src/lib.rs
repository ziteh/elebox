mod category;
mod comm;
mod csv;
mod default_db;
mod errors;
mod jamm_db;
mod manufacturer;
mod package;
mod part;
mod transfer;
mod yaml;

pub use category::*;
pub use comm::*;
pub use errors::*;
pub use jamm_db::*;
pub use manufacturer::*;
pub use package::*;
pub use part::*;

pub use default_db::create_default_db;
pub use transfer::export;
pub use transfer::import;

pub fn init(path: &str) {
    let part_mgr = PartManager::new(path);
    let _ = part_mgr.init();
}

pub fn check_db(path: &str) -> Result<(), String> {
    if let Err(err) = PackageManager::new(path).check() {
        return Err(ITEM_PART.to_string());
    }
    if let Err(err) = CategoryManager::new(path).check() {
        return Err(ITEM_CAT.to_string());
    }
    if let Err(err) = ManufacturerManager::new(path).check() {
        return Err(ITEM_MFR.to_string());
    }
    if let Err(err) = PackageManager::new(path).check() {
        return Err(ITEM_PKG.to_string());
    }
    Ok(())
}
