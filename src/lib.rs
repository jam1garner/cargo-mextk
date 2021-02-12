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
    install,
    run,
}

pub use build::SYMBOLS_PROPER_NAMES;

mod error;
pub use error::Error;

pub mod iso;
pub mod paths;
pub mod manifest;

#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Args {
    Mextk(SubCommands),
}

#[derive(StructOpt)]
pub enum SubCommands {
    #[structopt(about = "Create a new mod from the mextk template")]
    New {
        name: String,
    },
    
    #[structopt(about = "Build the current crate targetting MexTK")]
    Build {
        #[structopt(long)]
        debug: bool,
    },
    
    #[structopt(about = "Run the current crate targetting MexTK")]
    Run {
        #[structopt(long)]
        debug: bool,

        #[structopt(long)]
        no_restore: bool,
    },
    
    #[structopt(about = "Add an ISO to be managed")]
    AddIso {
        iso: PathBuf,
    },

    #[structopt(about = "Remove an ISO being managed by its id")]
    RemoveIso {
        id: String,
    },

    #[structopt(about = "List all ISOs being managed")]
    List,
    
    #[structopt(about = "Restore the extracted files for a given managed ISO provided its id")]
    Restore {
        id: String,
    },

    #[structopt(about = "Install the current crate to the mod directory")]
    Install {
        #[structopt(long)]
        restore: bool,
    },
}

pub fn main(args: Args) -> Result<(), Error> {
    let Args::Mextk(command) = args;

    match command {
        SubCommands::New { name } => new(&name),
        SubCommands::Build { debug } => {
            let _output = build(debug)?;

            //println!(
            //    "{}",
            //    format!("Object file built to {}", output.display())
            //        .bright_green()
            //        .bold()
            //);

            Ok(())
        },
        SubCommands::AddIso { iso } => iso::add(&iso, true),
        SubCommands::RemoveIso { id } => iso::remove(&id),
        SubCommands::List => iso::list().map(iso::display_list),
        SubCommands::Run { debug, no_restore } => run(debug, no_restore),
        SubCommands::Restore { id } => iso::restore(&id, true),
        SubCommands::Install { restore } => install(restore),
    }
}
