use clap::{Args, Parser, Subcommand};
use elebox_core;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Path to the `.db` file for the database
    #[arg(default_value = "elebox.db")]
    db: String,
    #[clap(subcommand)]
    entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
enum EntityType {
    /// Edit or list part
    Part(PartCommand),
    /// Edit or list part type
    Type(TypeCommand),
    /// Create and init a new database
    Init,
}

#[derive(Debug, Args)]
struct TypeCommand {
    #[clap(subcommand)]
    command: Option<TypeSubCommand>,
}

#[derive(Debug, Args)]
struct PartCommand {
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

#[derive(Debug, Subcommand)]
enum TypeSubCommand {
    /// Create a new part type
    New(NewTypeArgs),

    /// Remove a part type
    Delete(DeleteTypeArgs),

    /// Update a part type
    Update(UpdateTypeArgs),
}

#[derive(Debug, Args)]
struct NewPartArgs {
    name: String,
    quantity: u16,
    part_type: String,
    #[arg(short = 'm', long = "mfr")]
    mfr: Option<String>,
}

#[derive(Debug, Args)]
struct UpdateTypeArgs {
    old_name: String,
    new_name: Option<String>,
    #[arg(short = 'p', long = "parent")]
    parent_type: Option<String>,
}

#[derive(Debug, Args)]
struct UpdatePartArgs {
    old_name: String,
    new_name: Option<String>,
    new_quantity: Option<u16>,
    new_part_type: Option<String>,
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
struct NewTypeArgs {
    name: String,
    #[arg(short = 'p', long = "parent")]
    parent_type: Option<String>,
}

#[derive(Debug, Args)]
struct DeleteTypeArgs {
    name: String,
}

fn main() {
    let cli = Cli::parse();

    println!("{}", cli.db);

    match &cli.entity_type {
        EntityType::Init => elebox_core::init(&cli.db),
        EntityType::Part(p_cmd) => part_cmd(&cli.db, p_cmd),
        EntityType::Type(t_cmd) => type_cmd(&cli.db, t_cmd),
    };
}

fn part_cmd(db_path: &String, cmd: &PartCommand) {
    match &cmd.command {
        Some(subcmd) => match subcmd {
            PartSubCommand::New(args) => {
                let res =
                    elebox_core::add_part(db_path, &args.name, &args.quantity, &args.part_type);
                match res {
                    Ok(()) => println!("Add part {} x{}", args.name, args.quantity),
                    Err(err) => println!("{err}"),
                }
            }
            PartSubCommand::Delete(args) => {
                println!("Delete part {}", args.name);
            }
            PartSubCommand::Update(args) => {
                if let Err(err) = elebox_core::update_part(
                    db_path,
                    &args.old_name,
                    args.new_name.as_ref(),
                    args.new_quantity,
                    args.new_part_type.as_ref(),
                ) {
                    println!("{err}");
                }
            }
            PartSubCommand::Add(args) => {
                if let Err(err) =
                    elebox_core::update_part_quantity(db_path, &args.name, args.quantity as i16)
                {
                    println!("{err}");
                }
            }
            PartSubCommand::Use(args) => {
                let q = args.quantity as i16 * -1;
                if let Err(err) = elebox_core::update_part_quantity(db_path, &args.name, q) {
                    println!("{err}");
                }
            }
        },
        None => {
            println!("List part");
            let parts = elebox_core::get_parts(db_path);
            for part in parts {
                println!("{}   {}   {}", part.name, part.quantity, part.part_type);
            }
        }
    }
}

fn type_cmd(db_path: &String, cmd: &TypeCommand) {
    match &cmd.command {
        Some(TypeSubCommand::New(args)) => {
            if let Err(err) =
                elebox_core::add_part_type(db_path, &args.name, args.parent_type.as_ref())
            {
                println!("{err}");
            };
        }
        Some(TypeSubCommand::Delete(args)) => todo!(),
        Some(TypeSubCommand::Update(args)) => {
            if let Err(err) = elebox_core::update_part_type(
                db_path,
                &args.old_name,
                args.new_name.as_ref(),
                args.parent_type.as_ref(),
            ) {
                println!("{err}");
            };
        }
        None => {
            println!("List part type");
            let pts = elebox_core::get_part_types(db_path);
            for pt in pts {
                println!("{}  {}", pt.name, pt.parent);
            }
        }
    }
}
