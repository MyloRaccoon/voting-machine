use clap::Parser;
use clap::ValueEnum;

#[derive(Parser, Debug)]
pub struct Configuration {

	#[arg(short, long, value_delimiter = ' ', num_args = 1..)]
	pub candidates: Vec<String>,

	#[arg(short, long)]
	pub storage: StorageType,

	#[arg(short, long)]
	pub language: Language,

	#[arg(long)]
	pub service: Service,

	#[arg(short, long)]
	pub port: Option<u16>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum StorageType {
    File,
    Memory,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Language {
    En,
    Fr,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Service {
    Stdio,
    Udp,
}