mod category;
mod db;
mod errors;
mod part;

pub use category::*;
pub use errors::*;
pub use part::*;

pub fn init(path: &str) {
    db::init_db(path)
}
