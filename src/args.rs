use clap::{Parser, Subcommand};

pub mod bisect;

/// The tool's CLI arguments.
#[derive(Parser, Eq, PartialEq, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Dummy argument to enable git subcommand integration.
    ///
    /// This hidden argument allows `git-assist` to be invoked as `git assist`
    /// via git's subcommand mechanism. When git encounters a command like
    /// `git assist`, it looks for an executable named `git-assist` and invokes
    /// it with "assist" as the first argument. This dummy field consumes that
    /// argument, allowing the rest of the CLI to work normally.
    ///
    /// The argument is hidden from help output and only accepts "assist" as a value.
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
