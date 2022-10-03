/// Commandline options-related types
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// metropoli runtime host
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

impl Options {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Run(RunOptions),
}

/// Run a metropoli app
#[derive(Debug, Parser)]
#[clap()]
pub struct RunOptions {
    /// The path to a polis WASM file
    #[clap()]
    path: PathBuf,
}
