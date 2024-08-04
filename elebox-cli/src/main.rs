use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use elebox_core::{self, JammDatabase};

mod category_cmd;
mod manufacturer_cmd;
mod package_cmd;
mod part_cmd;

pub use category_cmd::*;
pub use manufacturer_cmd::*;
pub use package_cmd::*;
pub use part_cmd::*;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Path to the database file
    #[arg(default_value = "elebox.db")]
    db_path: String,

    #[clap(subcommand)]
    entity_type: EntityType,
}

#[derive(Debug, Args)]
struct PathArgs {
    #[arg(default_value = "./")]
    path: String,
}

#[derive(Debug, Subcommand)]
enum EntityType {
    /// Create and init a new database
    Init,

    /// Edit or query parts
    Part(PartCommand),

    /// Edit or query categories
    Category(CategoryCommand),

    /// Edit or query packages
    Package(PackageCommand),

    /// Edit or query manufacturers
    Mfr(ManufacturerCommand),

    /// Export all data
    Export(PathArgs),

    /// Import all data
    Import(PathArgs),
}

fn main() {
    let cli = Cli::parse();
    let db_path = cli.db_path;

    let part_db = Box::new(JammDatabase::new(&db_path));
    let pkg_db = Box::new(JammDatabase::new(&db_path));
    let cat_db = Box::new(JammDatabase::new(&db_path));
    let mfr_db = Box::new(JammDatabase::new(&db_path));
    let manager = elebox_core::Manager::new(part_db, pkg_db, cat_db, mfr_db);

    let _ = match &cli.entity_type {
        EntityType::Init => manager.init(),
        EntityType::Part(cmd) => Ok(part_cmd(manager.part(), cmd)),
        EntityType::Category(cmd) => Ok(category_cmd(manager.category(), cmd)),
        EntityType::Mfr(cmd) => Ok(manufacturer_cmd(manager.manufacturer(), cmd)),
        EntityType::Package(cmd) => Ok(package_cmd(manager.package(), cmd)),
        EntityType::Export(args) => manager.export(&PathBuf::from(args.path.clone())),
        EntityType::Import(args) => manager.import(&PathBuf::from(args.path.clone())),
    };
}
