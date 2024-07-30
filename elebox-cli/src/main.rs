use clap::{Args, Parser, Subcommand};
use elebox_core::{self, JammDatabase};

mod category_cmd;
mod part_cmd;

pub use category_cmd::*;
pub use part_cmd::*;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Path to the `.db` file for the database
    #[arg(default_value = "elebox.db")]
    db_path: String,
    #[clap(subcommand)]
    entity_type: EntityType,
}

#[derive(Debug, Args)]
struct CsvArgs {
    #[arg(default_value = "./")]
    path: String,
}

#[derive(Debug, Subcommand)]
enum EntityType {
    /// Create and init a new database
    Init,

    /// Edit or list part
    Part(PartCommand),

    /// Edit or list part category
    Category(CategoryCommand),

    /// Export all data to CSVs
    Export(CsvArgs),

    /// Import all data from CSVs
    Import(CsvArgs),
}

fn main() {
    let cli = Cli::parse();
    let db_path = cli.db_path;
    println!("Database: {}", db_path);

    let part_db = Box::new(JammDatabase::new(&db_path));
    let pkg_db = Box::new(JammDatabase::new(&db_path));
    let cat_db = Box::new(JammDatabase::new(&db_path));
    let mfr_db = Box::new(JammDatabase::new(&db_path));
    let manager = elebox_core::Manager::new(part_db, pkg_db, cat_db, mfr_db);

    match &cli.entity_type {
        EntityType::Init => manager.init(),
        EntityType::Part(cmd) => Ok(part_cmd(manager.part(), cmd)),
        EntityType::Category(cmd) => Ok(category_cmd(manager.category(), cmd)),
        EntityType::Export(_args) => todo!(),
        EntityType::Import(_args) => todo!(),
    };
}
