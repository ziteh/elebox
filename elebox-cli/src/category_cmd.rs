use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::ops::Not;

use clap::{Args, Subcommand};
use elebox_core::Category;
use elebox_core::Handler;
use elebox_core::Manager;

#[derive(Debug, Args)]
pub struct CategoryCommand {
    #[clap(subcommand)]
    command: Option<CategorySubCommand>,
}

#[derive(Debug, Subcommand)]
enum CategorySubCommand {
    /// Add a new category to the database
    New(NewCategoryArgs),

    /// Get info about a specific category
    Get(NameCategoryArgs),

    /// Remove a category from the database
    Delete(DeleteCategoryArgs),

    /// Update info of an existing category
    Update(UpdateCategoryArgs),

    /// Export data
    Export(ExportCategoryArgs),
}

#[derive(Debug, Args)]
struct UpdateCategoryArgs {
    ori_name: String,

    #[arg(short = 'n', long = "name")]
    new_name: Option<String>,

    #[arg(short = 'p', long = "parent")]
    new_parent_cat: Option<String>,

    #[arg(short = 'a', long = "alias")]
    new_alias: Option<String>,
}

#[derive(Debug, Args)]
struct NewCategoryArgs {
    /// Category name
    name: String,

    /// The parent category
    #[arg(short = 'p', long = "parent")]
    parent_cat: Option<String>,

    /// Alternative name
    #[arg(short = 'a', long = "alias")]
    alias: Option<String>,
}

#[derive(Debug, Args)]
struct NameCategoryArgs {
    name: String,
}

#[derive(Debug, Args)]
struct DeleteCategoryArgs {
    name: String,

    /// Skip confirm, delete directly
    #[arg(short = 'Y', long = "yes")]
    yes: bool,
}

#[derive(Debug, Args)]
struct ExportCategoryArgs {
    #[arg(default_value = "elebox_export_categories.tsv")]
    path: String,
}

pub fn category_cmd(handler: elebox_core::CategoryHandler, cmd: &CategoryCommand) {
    match &cmd.command {
        Some(CategorySubCommand::New(args)) => {
            if let Err(err) = handler.add(&elebox_core::Category::new(
                &args.name,
                args.parent_cat.as_deref(),
                args.alias.as_deref(),
            )) {
                println!("Error: {err}");
            };
        }
        Some(CategorySubCommand::Get(args)) => match handler.get(&args.name) {
            Ok(cat) => {
                println!(
                    "Name: {}, Alias: {}, Parent: {}",
                    cat.name,
                    unwrap_none(&cat.alias),
                    unwrap_none(&cat.parent),
                )
            }
            Err(err) => println!("Error: {err}"),
        },
        Some(CategorySubCommand::Delete(args)) => {
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
        Some(CategorySubCommand::Update(args)) => {
            let ori_cat = handler.get(&args.ori_name).expect("Category not found");

            let parent = args
                .new_parent_cat
                .as_deref()
                .or_else(|| ori_cat.parent.as_deref());
            let alias = args
                .new_alias
                .as_deref()
                .or_else(|| ori_cat.alias.as_deref());

            let new_item = Category::new(
                args.new_name.as_deref().unwrap_or(&ori_cat.name),
                parent.filter(|&s| !s.is_empty()),
                alias.filter(|&s| !s.is_empty()),
            );

            handler.update(&args.ori_name, &new_item);
        }
        Some(CategorySubCommand::Export(_args)) => {
            todo!();
        }
        None => {
            let categories = handler.list().unwrap();
            for cat in categories {
                println!(
                    "{}  {}  {}",
                    cat.name,
                    unwrap_none(&cat.alias),
                    unwrap_none(&cat.parent),
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
