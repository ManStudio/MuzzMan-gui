use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Flags {
    #[arg(short, long)]
    pub config_path: Option<PathBuf>,
}

impl Default for Flags {
    fn default() -> Self {
        Self::parse()
    }
}
