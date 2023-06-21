use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]

pub struct Config {
    pub file: PathBuf,
}
