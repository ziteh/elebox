mod category;
mod db;
mod errors;
mod part;
mod package;

pub use category::*;
 pub use db::Datebase;
pub use db::JammDatebase;
pub use errors::*;
pub use part::*;

// pub fn new(db_path: &str) -> db::Datebase {
//     db::JammDatebase::new(db_path)
// }

pub fn init(db: &dyn db::Datebase) {
    db.init();
}
