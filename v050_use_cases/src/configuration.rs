use clap::Parser;
use clap::ValueEnum;

#[derive(Parser, Debug)]
pub struct Configuration {
	#[arg(short, long, value_delimiter = ' ', num_args = 1..)]
	pub candidates: Vec<String>,
	#[arg(short, long, value_delimiter = ' ', num_args = 1..)]
	pub storage: StorageType,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum StorageType {
    File,
    Memory,
}
