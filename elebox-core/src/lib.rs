mod db;
mod errors;
mod part;
mod part_type;

pub use errors::*;
pub use part::*;
pub use part_type::*;

pub fn init(path: &str) {
  db::init_db(path)
}
