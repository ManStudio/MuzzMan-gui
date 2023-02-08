use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Clone, Subcommand)]
pub enum Command {
    Download {
        url: String,
        location_id: Option<String>,
        name: Option<String>,
        #[arg(short, long)]
        download: bool,
    },
    Attach {
        element_id: String,
    },
}

#[derive(Parser)]
pub struct Flags {
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl Default for Flags {
    fn default() -> Self {
        Flags::parse()
    }
}
