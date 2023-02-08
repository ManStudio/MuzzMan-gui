use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Flags {
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

impl Default for Flags {
    fn default() -> Self {
        Self::parse()
    }
}
