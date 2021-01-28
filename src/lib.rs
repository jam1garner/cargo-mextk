use owo_colors::OwoColorize;
use structopt::StructOpt;

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
}

mod error;
pub use error::Error;


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
    }
}
