use clap::{App, AppSettings, Parser};
use clap_complete::{generate, Generator, Shell};
use std::io;

#[derive(Parser, Debug)]
#[clap(
    name = "example",
    about,
    version,
    author,
    global_setting = AppSettings::DeriveDisplayOrder,
)]
pub struct Cli {
    #[clap(short, long, parse(from_occurrences))]
    /// Verbose output
    pub verbose: u8,
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Parser, PartialEq, Eq, Debug)]
pub enum Subcommand {
    /// Run test command
    Test {
        #[clap(long)]
        /// Force tests
        force: bool,
    },
    /// Generate shell completion
    Completion {
        /// Shell to generate completion for
        #[clap(arg_enum)]
        shell: Shell,
    },
    /// Print environment information
    Env,
}

pub fn print_completion<G: Generator>(gen: G, app: &mut App) {
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}
