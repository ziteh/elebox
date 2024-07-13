use clap::{Args, Subcommand};

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

pub fn category_cmd(db: &dyn elebox_core::Database, cmd: &CategoryCommand) {
    let manager = elebox_core::CategoryManager::new(db);

    match &cmd.command {
        Some(CategorySubCommand::New(args)) => {
            if let Err(err) = manager.add(&elebox_core::Category::new(
                &args.name,
                args.parent_cat.as_deref(),
            )) {
                println!("{err}");
            };
        }
        Some(CategorySubCommand::Delete(args)) => {
            if let Err(err) = manager.delete(&args.name) {
                print!("{err}");
            };
        }
        Some(CategorySubCommand::Update(args)) => {
            if let Err(err) = manager.update(
                &args.old_name,
                args.new_name.as_deref(),
                args.parent_cat.as_deref(),
            ) {
                println!("{err}");
            };
        }
        Some(CategorySubCommand::Export(args)) => {
            let _ = manager.export_csv(&args.path);
        }
        None => {
            println!("List part category");
            let pts = manager.list();
            for pt in pts {
                println!("{}  {:?}", pt.name, pt.parent);
            }
        }
    }
}
