use owo_colors::{OwoColorize, Style};
use structopt::StructOpt;

use cargo_mextk::SYMBOLS_PROPER_NAMES;
use cargo_mextk::Error;

fn main() {
    if let Err(err) = cargo_mextk::main(cargo_mextk::Args::from_args()) {
        eprintln!(
            "{}", 
            format!("Error: {}", err)
                .if_stderr_tty(|text| text.style(
                    Style::new().bright_red().bold()
                ))
        );

        let suggestion = "Suggestion:".bright_cyan();
        let suggestion = suggestion.bold();
        
        let example = "Example:".bright_cyan();
        let example = example.bold();
        
        let more_info = "More Info:".bright_cyan();
        let more_info = more_info.bold();

        match err {
            Error::ExitStatus(code) => std::process::exit(code),
            Error::InvalidSymbolName => {

                println!("{} use one of the following", suggestion);
                for symbol in SYMBOLS_PROPER_NAMES {
                    println!("    symbols = \"{}\"", symbol);
                }
            }
            Error::NoSuchIso => {
                println!(
                    "{} Add an iso with `cargo mextk add-iso [iso]`",
                    suggestion
                );
                println!(
                    "{} cargo mextk add-iso GALE01.iso",
                    example
                );
                println!(
                    "{} use `cargo mextk list` to list your currently managed ISO files.",
                    more_info
                );
            },
            Error::InvalidGcm => {
                println!(
                    "{} Check if the file is in the NKIT format",
                    suggestion
                );
                println!(
                    "{}: https://wiki.gbatemp.net/wiki/NKit",
                    more_info,
                );
            },
            Error::NoDatName => {
                println!("{} Try adding `dat = \"...\"` to Mextk.toml.", suggestion);
                println!("{} dat = \"GrFs.dat\"", example);
            }
            _ => ()
        }
    }
}
