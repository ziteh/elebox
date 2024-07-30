use std::path::PathBuf;

use clap::{Args, Subcommand};
use elebox_core::{Handler, Manager, Transferable};

#[derive(Debug, Args)]
pub struct PartCommand {
    #[clap(subcommand)]
    command: Option<PartSubCommand>,
}

#[derive(Debug, Subcommand)]
enum PartSubCommand {
    /// Create a new part
    New(NewPartArgs),

    /// Remove a part
    Delete(DeletePartArgs),

    /// Update a part
    Update(UpdatePartArgs),

    /// Restock
    Add(AddPartArgs),

    /// Consume
    Use(UsePartArgs),

    /// Export as CSV file
    Export(CsvPartArgs),

    /// Import from CSV file
    Import(CsvPartArgs),
}

#[derive(Debug, Args)]
struct NewPartArgs {
    name: String,
    quantity: u16,
    category: String,
    #[arg(short = 'm', long = "mfr")]
    mfr: Option<String>,
}

#[derive(Debug, Args)]
struct UpdatePartArgs {
    old_name: String,
    new_name: Option<String>,
    new_quantity: Option<u16>,
    new_part_cat: Option<String>,
    #[arg(short = 'm', long = "mfr")]
    mfr: Option<String>,
}

#[derive(Debug, Args)]
struct AddPartArgs {
    name: String,
    quantity: u16,
}

#[derive(Debug, Args)]
struct UsePartArgs {
    name: String,
    quantity: u16,
}

#[derive(Debug, Args)]
struct DeletePartArgs {
    name: String,
}

#[derive(Debug, Args)]
struct CsvPartArgs {
    #[arg(default_value = "elebox_export_parts.yaml")]
    path: String,
}

// pub fn part_cmd(db: &dyn elebox_core::Database, cmd: &PartCommand) {
pub fn part_cmd(handler: elebox_core::PartHandler, cmd: &PartCommand) {
    match &cmd.command {
        Some(sub_cmd) => match sub_cmd {
            PartSubCommand::New(args) => {
                let res = handler.add(&elebox_core::Part::new(
                    &args.name,
                    &args.category,
                    args.quantity,
                ));
                match res {
                    Ok(()) => println!("Add part {} x{}", args.name, args.quantity),
                    Err(err) => println!("{err}"),
                }
            }
            PartSubCommand::Delete(args) => {
                if let Err(err) = handler.delete(&args.name) {
                    println!("ERR: {err}");
                }
            }
            PartSubCommand::Update(_args) => {
                todo!();
            }
            PartSubCommand::Add(args) => {
                if let Err(err) = handler.update_part_quantity(&args.name, args.quantity as i16) {
                    println!("ERR: {err}");
                }
            }
            PartSubCommand::Use(args) => {
                let q = args.quantity as i16 * -1;
                if let Err(err) = handler.update_part_quantity(&args.name, q) {
                    println!("ERR: {err}");
                }
            }
            PartSubCommand::Export(args) => match handler.export(&PathBuf::from(&args.path)) {
                Ok(_) => println!("Export success: {}", args.path),
                Err(_) => println!("Export error: {}", args.path),
            },
            PartSubCommand::Import(args) => match handler.import(&PathBuf::from(&args.path)) {
                Ok(_) => println!("Import success: {}", args.path),
                Err(_) => println!("Import error: {}", args.path),
            },
        },
        None => {
            println!("List part");
            let parts = handler.list().unwrap();
            for part in parts {
                println!("{}   {}   {}", part.name, part.quantity, part.category);
            }
        }
    }
}
