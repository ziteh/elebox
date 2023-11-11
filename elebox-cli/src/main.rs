use clap::{Args, Parser, Subcommand};
use elebox_core;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    entity_type: Option<EntityType>,
}

#[derive(Debug, Subcommand)]
enum EntityType {
    Part(PartCommand),
    Type(TypeCommand),
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

    /// Retrieve
    Take(TakePartArgs),
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
    name: String,
    quantity: u16,
    part_type: String,
    #[arg(short = 'm', long = "mfr")]
    mfr: Option<String>,
}

#[derive(Debug, Args)]
struct AddPartArgs {
    name: String,
    quantity: u16,
}

#[derive(Debug, Args)]
struct TakePartArgs {
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
    match &cli.entity_type {
        Some(EntityType::Part(p_cmd)) => part_cmd(p_cmd),
        Some(EntityType::Type(t_cmd)) => type_cmd(t_cmd),
        None => {}
    };
}

fn part_cmd(cmd: &PartCommand) {
    match &cmd.command {
        Some(subcmd) => match subcmd {
            PartSubCommand::New(args) => {
                let res = elebox_core::add_part(&args.name, &args.quantity, &args.part_type);
                match res {
                    Ok(()) => println!("Add part {} x{}", args.name, args.quantity),
                    Err(err) => println!("{err}"),
                }
            }
            PartSubCommand::Delete(args) => {
                println!("Delete part {}", args.name);
            }
            PartSubCommand::Update(args) => todo!(),
            PartSubCommand::Add(args) => todo!(),
            PartSubCommand::Take(args) => todo!(),
        },
        None => {
            println!("List part");
            let parts = elebox_core::get_parts();
            for part in parts {
                println!("{}   {}   {}", part.name, part.quantity, part.part_type);
            }
        }
    }
}

fn type_cmd(cmd: &TypeCommand) {
    match &cmd.command {
        Some(TypeSubCommand::New(args)) => {
            if let Err(err) = elebox_core::add_part_type(&args.name, args.parent_type.as_ref()) {
                println!("{err}");
            };
        }
        Some(TypeSubCommand::Delete(args)) => todo!(),
        Some(TypeSubCommand::Update(args)) => {
            if let Err(err) = elebox_core::update_part_type(
                &args.old_name,
                args.new_name.as_ref(),
                args.parent_type.as_ref(),
            ) {
                println!("{err}");
            };
        }
        None => {
            println!("List part type");
            let pts = elebox_core::get_part_types();
            for pt in pts {
                println!("{}  {}", pt.name, pt.parent);
            }
        }
    }
}
