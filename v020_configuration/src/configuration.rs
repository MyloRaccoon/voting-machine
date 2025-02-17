use clap::Parser;

#[derive(Parser, Debug)]
pub struct Configuration {
	#[arg(short, long, value_delimiter = ' ', num_args = 1..)]
	pub candidates: Vec<String>,
}
