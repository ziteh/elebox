use clap::{Args, Subcommand};
use elebox_core::Category;
use elebox_core::Handler;
use elebox_core::Manager;
use elebox_core::Manufacturer;
use elebox_core::Package;
use elebox_core::PackageType;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::ops::Not;

#[derive(Debug, Args)]
pub struct PackageCommand {
    #[clap(subcommand)]
    command: Option<PackageSubCommand>,
}

#[derive(Debug, Subcommand)]
enum PackageSubCommand {
    /// Add a new package to the database
    New(NewArgs),

    /// Get info about a specific package
    Get(NameArgs),

    /// Remove a package from the database
    Delete(DeleteArgs),

    /// Update info of an existing package
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

    #[arg(short = 't', long = "type")]
    new_type: Option<String>, // TODO enum
}

#[derive(Debug, Args)]
struct NewArgs {
    /// Package name
    name: String,

    /// Package technology
    pkg_type: String,

    /// Alternative name
    #[arg(short = 'a', long = "alias")]
    alias: Option<String>,
}

#[derive(Debug, Args)]
struct NameArgs {
    name: String,
}

#[derive(Debug, Args)]
struct DeleteArgs {
    name: String,

    /// Skip confirm, delete directly
    #[arg(short = 'Y', long = "yes")]
    yes: bool,
}

#[derive(Debug, Args)]
struct ExportArgs {
    #[arg(default_value = "elebox_export_packages.tsv")] // TODO filename
    path: String,
}

pub fn package_cmd(handler: elebox_core::PackageHandler, cmd: &PackageCommand) {
    match &cmd.command {
        Some(PackageSubCommand::New(args)) => {
            if let Err(err) = handler.add(&elebox_core::Package::new(
                &args.name,
                to_app_type(&args.pkg_type),
                args.alias.as_deref(),
            )) {
                println!("Error: {err}");
            };
        }
        Some(PackageSubCommand::Get(args)) => match handler.get(&args.name) {
            Ok(mfr) => {
                println!(
                    "Name: {}, Type: {}, Alias: {}",
                    mfr.name,
                    &mfr.pkg_type.to_string(),
                    unwrap_none(&mfr.alias),
                )
            }
            Err(err) => println!("Error: {err}"),
        },
        Some(PackageSubCommand::Delete(args)) => {
            // Confirm delete message
            if !args.yes {
                println!("Are you sure you want to delete '{}' ?", args.name);
                print!("This action cannot be undone. [y/N]: ");

                let mut input = String::new();
                let _ = stdout().flush();
                stdin().read_line(&mut input).expect("Failed to read input");

                if input.trim_end().to_lowercase() != String::from("y") {
                    println!("Deletion canceled");
                    return;
                }
            }

            println!("Deleting '{}'...", args.name);
            if let Err(err) = handler.delete(&args.name) {
                println!("Error: {err}");
            };
        }
        Some(PackageSubCommand::Update(args)) => {
            let ori_pkg = handler.get(&args.ori_name).expect("Package not found");

            let pkg_type = match &args.new_type {
                Some(pt) => to_app_type(&pt),
                None => ori_pkg.pkg_type,
            };

            let alias = args
                .new_alias
                .as_deref()
                .or_else(|| ori_pkg.alias.as_deref());

            let new_item = Package::new(
                args.new_name.as_deref().unwrap_or(&ori_pkg.name),
                pkg_type,
                alias.filter(|&s| !s.is_empty()),
            );

            handler.update(&args.ori_name, &new_item);
        }
        Some(PackageSubCommand::Export(_args)) => {
            todo!();
        }
        None => {
            let mfrs = handler.list().unwrap();
            for mfr in mfrs {
                println!(
                    "{}  {}  {}",
                    mfr.name,
                    &mfr.pkg_type.to_string(),
                    unwrap_none(&mfr.alias),
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

fn to_app_type(cmd_type: &str) -> PackageType {
    match cmd_type.to_lowercase().as_str() {
        "smt" => PackageType::Smt,
        "tht" => PackageType::Tht,
        _ => PackageType::Others,
    }
}
