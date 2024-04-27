mod category;
mod db;
mod errors;
mod part;

pub use category::*;
pub use errors::*;
pub use part::*;
pub use db::Datebase;

pub fn new(db_path: &str) -> db::Datebase {
    db::Datebase::new(db_path)
}

pub fn init(db: db::Datebase) {
    db.init();
}
