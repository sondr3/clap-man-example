use clap::{IntoApp, Parser};
use cli::Cli;

use crate::cli::print_completion;

mod cli;

fn main() {
    let cli = Cli::parse();

    println!("{:#?}", cli);

    match cli.subcommand {
        cli::Subcommand::Test { .. } => println!("Testing..."),
        cli::Subcommand::Completion { shell } => print_completion(shell, &mut Cli::into_app()),
        cli::Subcommand::Env => println!("Env..."),
    }
}
