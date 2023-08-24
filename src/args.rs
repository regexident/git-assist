use clap::{Parser, Subcommand};

pub mod bisect;

/// The tool's CLI arguments.
#[derive(Parser, Eq, PartialEq, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(hide = true, value_parser = clap::builder::PossibleValuesParser::new(["assist"]))]
    pub(crate) dummy: Option<String>,

    #[command(subcommand)]
    pub(crate) command: Option<Command>,
}

#[derive(Parser, Eq, PartialEq, Debug)]
pub(crate) struct CommonOptions {
    // /// Verbose mode.
    // #[arg(long)]
    // pub(crate) verbose: bool,
}

#[derive(Subcommand, Eq, PartialEq, Debug)]
pub(crate) enum Command {
    #[command(subcommand)]
    Bisect(bisect::Command),
}
