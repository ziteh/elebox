use std::sync::atomic;

use clap::{Args, Parser, Subcommand};
use elebox_core::{self};

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

    println!("Database: {}", cli.db_path);

    match &cli.entity_type {
        EntityType::Init => elebox_core::init(&cli.db_path),
        EntityType::Part(cmd) => part_cmd(&cli.db_path, cmd),
        EntityType::Category(cmd) => category_cmd(&cli.db_path, cmd),
        EntityType::Export(args) => todo!(),
        EntityType::Import(args) => todo!(),
    };
}
