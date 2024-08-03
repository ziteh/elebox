use clap::{Args, Subcommand};
use elebox_core::Category;
use elebox_core::Handler;
use elebox_core::Manager;
use elebox_core::Manufacturer;

#[derive(Debug, Args)]
pub struct ManufacturerCommand {
    #[clap(subcommand)]
    command: Option<ManufacturerSubCommand>,
}

#[derive(Debug, Subcommand)]
enum ManufacturerSubCommand {
    /// Add a new manufacturer to the database
    New(NewArgs),

    /// Get info about a specific manufacturer
    Get(NameArgs),

    /// Remove a manufacturer from the database
    Delete(NameArgs),

    /// Update info of an existing manufacturer
    Update(UpdateArgs),

    /// Export data
    Export(ExportArgs),
}

#[derive(Debug, Args)]
struct UpdateArgs {
    ori_name: String,

    #[arg(short = 'n', long = "name")]
    new_name: Option<String>,

    #[arg(short = 'a', long = "alias")]
    new_alias: Option<String>,

    #[arg(short = 'l', long = "link")]
    new_link: Option<String>,
}

#[derive(Debug, Args)]
struct NewArgs {
    /// Manufacturer name
    name: String,

    /// Alternative name
    #[arg(short = 'a', long = "alias")]
    alias: Option<String>,

    /// URL of the manufacturer
    #[arg(short = 'l', long = "link")]
    link: Option<String>,
}

#[derive(Debug, Args)]
struct NameArgs {
    name: String,
}

#[derive(Debug, Args)]
struct ExportArgs {
    #[arg(default_value = "elebox_export_mfrs.tsv")] // TODO filename
    path: String,
}

pub fn manufacturer_cmd(handler: elebox_core::ManufacturerHandler, cmd: &ManufacturerCommand) {
    match &cmd.command {
        Some(ManufacturerSubCommand::New(args)) => {
            if let Err(err) = handler.add(&elebox_core::Manufacturer::new(
                &args.name,
                args.alias.as_deref(),
                args.link.as_deref(),
            )) {
                println!("Error: {err}");
            };
        }
        Some(ManufacturerSubCommand::Get(args)) => match handler.get(&args.name) {
            Ok(mfr) => {
                println!(
                    "Name: {}, Alias: {}, Link: {}",
                    mfr.name,
                    unwrap_none(&mfr.alias),
                    unwrap_none(&mfr.url),
                )
            }
            Err(err) => println!("Error: {err}"),
        },
        Some(ManufacturerSubCommand::Delete(args)) => {
            if let Err(err) = handler.delete(&args.name) {
                println!("Error: {err}");
            };
        }
        Some(ManufacturerSubCommand::Update(args)) => {
            let ori_mfr = handler.get(&args.ori_name).expect("Manufacturer not found");

            let link = args.new_link.as_deref().or_else(|| ori_mfr.url.as_deref());
            let alias = args
                .new_alias
                .as_deref()
                .or_else(|| ori_mfr.alias.as_deref());

            let new_item = Manufacturer::new(
                args.new_name.as_deref().unwrap_or(&ori_mfr.name),
                alias.filter(|&s| !s.is_empty()),
                link.filter(|&s| !s.is_empty()),
            );

            handler.update(&args.ori_name, &new_item);
        }
        Some(ManufacturerSubCommand::Export(_args)) => {
            todo!();
        }
        None => {
            let mfrs = handler.list().unwrap();
            for mfr in mfrs {
                println!(
                    "{}  {}  {}",
                    mfr.name,
                    unwrap_none(&mfr.alias),
                    unwrap_none(&mfr.url),
                );
            }
        }
    }
}

fn unwrap_none(val: &Option<String>) -> String {
    match val {
        Some(v) => String::from(v),
        None => String::from("-none-"),
    }
}
