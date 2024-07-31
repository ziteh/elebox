use std::{net, path::PathBuf};

use clap::{Args, Subcommand};
use elebox_core::{Handler, Manager, Part, Transferable};

#[derive(Debug, Args)]
pub struct PartCommand {
    #[clap(subcommand)]
    command: Option<PartSubCommand>,
}

#[derive(Debug, Subcommand)]
enum PartSubCommand {
    /// Create a new part
    New(NewPartArgs),

    /// Get a part
    Get(NamePartArgs),

    /// Remove a part
    Delete(NamePartArgs),

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

    #[arg(short = 'p', long = "package")]
    package: Option<String>,

    #[arg(short = 'P', long = "package-detail")]
    package_detail: Option<String>,

    #[arg(short = 'M', long = "mfr-number")]
    mfr_no: Option<String>,

    #[arg(short = 'a', long = "alias")]
    alias: Option<String>,

    #[arg(short = 'l', long = "location")]
    location: Option<String>,

    #[arg(short = 'r', long = "product")]
    product: Option<String>,

    #[arg(short = 'D', long = "datasheet")]
    datasheet: Option<String>,

    #[arg(short = 'i', long = "image")]
    image: Option<String>,

    #[arg(short = 'd', long = "description")]
    description: Option<String>,

    #[arg(short = 's', long = "starred")]
    starred: bool,
    // TODO custom field and suppliers
}

#[derive(Debug, Args)]
struct UpdatePartArgs {
    ori_name: String,

    #[arg(short = 'n', long = "name")]
    name: Option<String>,

    #[arg(short = 'q', long = "quantity")]
    quantity: Option<u16>,

    #[arg(short = 'c', long = "category")]
    category: Option<String>,

    #[arg(short = 'm', long = "mfr")]
    mfr: Option<String>,

    #[arg(short = 'p', long = "package")]
    package: Option<String>,

    #[arg(short = 'P', long = "package-detail")]
    package_detail: Option<String>,

    #[arg(short = 'M', long = "mfr-number")]
    mfr_no: Option<String>,

    #[arg(short = 'a', long = "alias")]
    alias: Option<String>,

    #[arg(short = 'l', long = "location")]
    location: Option<String>,

    #[arg(short = 'r', long = "product")]
    product: Option<String>,

    #[arg(short = 'D', long = "datasheet")]
    datasheet: Option<String>,

    #[arg(short = 'i', long = "image")]
    image: Option<String>,

    #[arg(short = 'd', long = "description")]
    description: Option<String>,

    #[arg(short = 's', long = "starred")]
    starred: Option<bool>,
    // TODO custom field and suppliers
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
struct NamePartArgs {
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
                let part = Part {
                    name: args.name.clone(),
                    quantity: args.quantity,
                    category: args.category.clone(),
                    package: args.package.clone(),
                    package_detail: args.package_detail.clone(),
                    alias: args.alias.clone(),
                    description: args.description.clone(),
                    location: args.location.clone(),
                    mfr: args.mfr.clone(),
                    mfr_no: args.mfr_no.clone(),
                    datasheet_link: args.datasheet.clone(),
                    product_link: args.product.clone(),
                    image_link: args.image.clone(),
                    starred: args.starred,
                    custom_fields: vec![], // TODO
                    suppliers: vec![],     // TODO
                };

                let res = handler.add(&part);
                match res {
                    Ok(()) => println!("Add part {} x{}", args.name, args.quantity),
                    Err(err) => println!("{err}"),
                }
            }
            PartSubCommand::Get(args) => match handler.get(&args.name) {
                Ok(part) => {
                    println!(
                        "Name: {}\n\
                        Quantity: {}\n\
                        Category: {}\n\
                        Package: {}\n\
                        Package Detail: {}\n\
                        Alias: {}\n\
                        Description: {}\n\
                        Location: {}\n\
                        Manufacturer: {}\n\
                        Manufacturer Number: {}\n\
                        Datasheet Link: {}\n\
                        Product Link: {}\n\
                        Image Link: {}\n\
                        Starred: {}",
                        part.name,
                        part.quantity,
                        part.category,
                        unwrap_none(&part.package),
                        unwrap_none(&part.package_detail),
                        unwrap_none(&part.alias),
                        unwrap_none(&part.description),
                        unwrap_none(&part.location),
                        unwrap_none(&part.mfr),
                        unwrap_none(&part.mfr_no),
                        unwrap_none(&part.datasheet_link),
                        unwrap_none(&part.product_link),
                        unwrap_none(&part.image_link),
                        part.starred,
                    )
                }
                Err(err) => println!("Error: {err}"),
            },
            PartSubCommand::Delete(args) => {
                if let Err(err) = handler.delete(&args.name) {
                    println!("ERR: {err}");
                }
            }
            PartSubCommand::Update(args) => {
                let ori_part = handler.get(&args.ori_name).expect("Not found");

                let name = match &args.name {
                    Some(n) => n.clone(),
                    None => ori_part.name.clone(),
                };

                let category = match &args.category {
                    Some(c) => c.clone(),
                    None => ori_part.category.clone(),
                };

                let quantity = match args.quantity {
                    Some(q) => q,
                    None => ori_part.quantity,
                };

                let starred = match args.starred {
                    Some(s) => s,
                    None => ori_part.starred,
                };

                // TODO
                let package = args
                    .package
                    .as_deref()
                    .or_else(|| ori_part.package.as_deref())
                    .filter(|&s| !s.is_empty())
                    .map(String::from);
                let package_detail = args
                    .package_detail
                    .as_deref()
                    .or_else(|| ori_part.package_detail.as_deref())
                    .filter(|&s| !s.is_empty())
                    .map(String::from);
                let alias = args
                    .alias
                    .as_deref()
                    .or_else(|| ori_part.alias.as_deref())
                    .filter(|&s| !s.is_empty())
                    .map(String::from);
                let description = args
                    .description
                    .as_deref()
                    .or_else(|| ori_part.description.as_deref())
                    .filter(|&s| !s.is_empty())
                    .map(String::from);
                let location = args
                    .location
                    .as_deref()
                    .or_else(|| ori_part.location.as_deref())
                    .filter(|&s| !s.is_empty())
                    .map(String::from);
                let mfr = args
                    .mfr
                    .as_deref()
                    .or_else(|| ori_part.mfr.as_deref())
                    .filter(|&s| !s.is_empty())
                    .map(String::from);
                let mfr_no = args
                    .mfr_no
                    .as_deref()
                    .or_else(|| ori_part.mfr_no.as_deref())
                    .filter(|&s| !s.is_empty())
                    .map(String::from);
                let datasheet_link = args
                    .datasheet
                    .as_deref()
                    .or_else(|| ori_part.datasheet_link.as_deref())
                    .filter(|&s| !s.is_empty())
                    .map(String::from);
                let product_link = args
                    .product
                    .as_deref()
                    .or_else(|| ori_part.product_link.as_deref())
                    .filter(|&s| !s.is_empty())
                    .map(String::from);
                let image_link = args
                    .image
                    .as_deref()
                    .or_else(|| ori_part.image_link.as_deref())
                    .filter(|&s| !s.is_empty())
                    .map(String::from);

                let new_item = Part {
                    name,
                    quantity,
                    category,
                    package,
                    package_detail,
                    alias,
                    description,
                    location,
                    mfr,
                    mfr_no,
                    datasheet_link,
                    product_link,
                    image_link,
                    starred,
                    custom_fields: ori_part.custom_fields.clone(), // TODO
                    suppliers: ori_part.suppliers.clone(),         // TODO
                };

                handler.update(&args.ori_name, &new_item);
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

fn unwrap_none(val: &Option<String>) -> String {
    match val {
        Some(v) => String::from(v),
        None => String::from("-none-"),
    }
}
