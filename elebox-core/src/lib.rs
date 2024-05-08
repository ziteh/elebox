mod category;
mod db;
mod errors;
mod manufacturer;
mod package;
mod part;

pub use category::*;
pub use db::Datebase;
pub use db::JammDatebase;
pub use errors::*;
pub use manufacturer::*;
pub use package::*;
pub use part::*;

// pub fn new(db_path: &str) -> db::Datebase {
//     db::JammDatebase::new(db_path)
// }

pub fn init(db: &dyn db::Datebase) {
    db.init();
}
