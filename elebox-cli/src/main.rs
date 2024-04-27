use clap::{Parser, Subcommand};
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

#[derive(Debug, Subcommand)]
enum EntityType {
    /// Create and init a new database
    Init,

    /// Edit or list part
    Part(PartCommand),

    /// Edit or list part category
    Category(CategoryCommand),
}

fn main() {
    let cli = Cli::parse();

    println!("{}", cli.db_path);
    let db = elebox_core::new(&cli.db_path);

    match &cli.entity_type {
        EntityType::Init => elebox_core::init(db),
        EntityType::Part(cmd) => part_cmd(db, cmd),
        EntityType::Category(cmd) => category_cmd(db, cmd),
    };
}
