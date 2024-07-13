use std::sync::atomic;

use clap::{Args, Parser, Subcommand};
use elebox_core::{self, Database};

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

    println!("{}", cli.db_path);
    let db = elebox_core::JammDatabase::new(&cli.db_path);

    match &cli.entity_type {
        EntityType::Init => db.init(),
        EntityType::Part(cmd) => part_cmd(&db, cmd),
        EntityType::Category(cmd) => category_cmd(&db, cmd),
        EntityType::Export(args) => elebox_core::export_csv(&db, &args.path),
        EntityType::Import(args) => {
            let _ = elebox_core::import_csv(&args.path);
        }
    };
}
