use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "pullpiri")]
#[command(bin_name = "pullpiri")]
pub struct PiccoloCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Status,
}

pub fn parse() -> PiccoloCli {
    PiccoloCli::parse()
}
