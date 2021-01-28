use owo_colors::{OwoColorize, Style};
use structopt::StructOpt;

fn main() {
    if let Err(err) = cargo_mextk::main(cargo_mextk::Args::from_args()) {
        eprintln!(
            "{}", 
            format!("Error: {}", err)
                .if_stderr_tty(|text| text.style(
                    Style::new().bright_red().bold()
                ))
        );

        if let cargo_mextk::Error::ExitStatus(code) = err {
            std::process::exit(code)
        }
    }
}
