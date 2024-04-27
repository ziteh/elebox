use clap::{Args, Subcommand};

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

pub fn part_cmd(db: &dyn elebox_core::Datebase, cmd: &PartCommand) {
    let manager = elebox_core::PartManager::new(db);

    match &cmd.command {
        Some(subcmd) => match subcmd {
            PartSubCommand::New(args) => {
                let res = manager.add(&elebox_core::Part::new(
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
                println!("Delete part {}", args.name);
            }
            PartSubCommand::Update(args) => {
                if let Err(err) = manager.update(
                    &args.old_name,
                    args.new_name.as_deref(),
                    args.new_quantity,
                    args.new_part_cat.as_deref(),
                ) {
                    println!("{err}");
                }
            }
            PartSubCommand::Add(args) => {
                if let Err(err) = manager.update_part_quantity(&args.name, args.quantity as i16) {
                    println!("{err}");
                }
            }
            PartSubCommand::Use(args) => {
                let q = args.quantity as i16 * -1;
                if let Err(err) = manager.update_part_quantity(&args.name, q) {
                    println!("{err}");
                }
            }
        },
        None => {
            println!("List part");
            let parts = manager.list();
            for part in parts {
                println!("{}   {}   {}", part.name, part.quantity, part.catrgory);
            }
        }
    }
}
