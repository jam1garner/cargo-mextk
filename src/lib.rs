use owo_colors::OwoColorize;
use structopt::StructOpt;
use std::path::PathBuf;

macro_rules! subcommands {
    ($($ident:ident),* $(,)?) => {
        $(
            mod $ident;
            pub use $ident::*;
        )*
    };
}

subcommands!{
    new,
    build,
    run,
}

mod error;
pub use error::Error;

pub mod iso;
pub mod paths;

#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Args {
    Mextk(SubCommands),
}

#[derive(StructOpt)]
pub enum SubCommands {
    #[structopt(help = "Create a new mod from the mextk template")]
    New {
        name: String,
    },
    
    #[structopt(help = "Build the current crate targetting MexTK")]
    Build {
        #[structopt(long)]
        debug: bool,
    },
    
    #[structopt(help = "Add an ISO to be managed")]
    AddIso {
        iso: PathBuf,
    },

    #[structopt(help = "Remove an ISO being managed by its id")]
    RemoveIso {
        id: String,
    },

    #[structopt(help = "List all ISOs being managed")]
    List,
}

pub fn main(args: Args) -> Result<(), Error> {
    let Args::Mextk(command) = args;

    match command {
        SubCommands::New { name } => new(&name),
        SubCommands::Build { debug } => {
            let output = build(debug)?;

            println!(
                "{}",
                format!("Object file built to {}", output.display())
                    .bright_green()
                    .bold()
            );

            Ok(())
        },
        SubCommands::AddIso { iso } => iso::add(&iso, true),
        SubCommands::RemoveIso { id } => iso::remove(&id),
        SubCommands::List => iso::list().map(iso::display_list),
        //SubCommands::Build { debug } => run(debug)
    }
}
