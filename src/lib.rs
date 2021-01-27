use structopt::StructOpt;

mod new;

#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Args {
    Skyline(SubCommands),
}

#[derive(StructOpt)]
pub enum SubCommands {
    #[structopt(help = "Create a new mod from the mextk template")]
    New {
        name: String,
    }
}

pub struct Error;

pub fn main(args: Args) -> Result<(), Error> {
    let Args::Skyline(command) = args;

    match command {
        SubCommands::New { name } => todo!(),
    }
    
    Ok(())
}
