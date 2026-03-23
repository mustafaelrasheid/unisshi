use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "unisshi")]
#[command(version = "0.1.0")]
#[command(author = "mustafaelrasheid")]
#[command(
    about = "diary and logging cli app",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Today,
    AddThought,
    Recheck
}
