use clap::{Parser, Subcommand};

// !TODO: Add Path option

#[derive(Debug, Subcommand)]
pub enum Command {
    Install,
    Uninstall,
}

#[derive(Debug, Parser)]
pub struct Flags {
    #[command(subcommand)]
    pub command: Option<Command>,
    #[arg(short, long)]
    pub local: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Flags::parse()
    }
}
