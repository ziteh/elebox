use clap::{Args, Subcommand};
use elebox_core::Manager;

#[derive(Debug, Args)]
pub struct CategoryCommand {
    #[clap(subcommand)]
    command: Option<CategorySubCommand>,
}

#[derive(Debug, Subcommand)]
enum CategorySubCommand {
    /// Create a new category
    New(NewCategoryArgs),

    /// Remove a category
    Delete(DeleteCategoryArgs),

    /// Update a category
    Update(UpdateCategoryArgs),

    /// Export CSV file
    Export(ExportCategoryArgs),
}

#[derive(Debug, Args)]
struct UpdateCategoryArgs {
    old_name: String,
    new_name: Option<String>,
    #[arg(short = 'p', long = "parent")]
    parent_cat: Option<String>,
}

#[derive(Debug, Args)]
struct NewCategoryArgs {
    name: String,
    #[arg(short = 'p', long = "parent")]
    parent_cat: Option<String>,
}

#[derive(Debug, Args)]
struct DeleteCategoryArgs {
    name: String,
}

#[derive(Debug, Args)]
struct ExportCategoryArgs {
    #[arg(default_value = "elebox_export_categories.tsv")]
    path: String,
}

pub fn category_cmd(path: &str, cmd: &CategoryCommand) {
    let manager = elebox_core::CategoryManager::new(path);

    match &cmd.command {
        Some(CategorySubCommand::New(args)) => {
            if let Err(err) = manager.add(&elebox_core::Category::new(
                &args.name,
                args.parent_cat.as_deref(),
                None, // TODO
            )) {
                println!("ERR: {err}");
            };
        }
        Some(CategorySubCommand::Delete(args)) => {
            if let Err(err) = manager.delete(&args.name) {
                println!("ERR: {err}");
            };
        }
        Some(CategorySubCommand::Update(_args)) => {
            todo!();
        }
        Some(CategorySubCommand::Export(_args)) => {
            todo!();
        }
        None => {
            println!("List part category");
            let pts = manager.list().unwrap();
            for pt in pts {
                println!("{}  {:?}", pt.name, pt.parent);
            }
        }
    }
}
