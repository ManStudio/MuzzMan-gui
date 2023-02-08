use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    Info { element_id: String },
}

#[derive(Parser, Debug)]
pub struct Flags {
    #[command(subcommand)]
    pub command: Command,
    #[arg(short, long)]
    pub config_path: Option<PathBuf>,
}

impl Default for Flags {
    fn default() -> Self {
        Self::parse()
    }
}
