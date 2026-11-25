use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "unisshi")]
#[command(version)]
#[command(author = "mustafaelrasheid")]
#[command(
	about = "A personal diary and logging cli app.",
	long_about = None
)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	Today {
		#[arg(long)]
		overwrite: bool
	},
	Thought,
	Recheck,
}
