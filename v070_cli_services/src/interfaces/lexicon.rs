#[derive(Eq, PartialEq, Clone)]
pub struct Lexicon {
	pub welcome: &'static str,
	pub please: &'static str,

	pub vote_command: &'static str,
	pub voters_command: &'static str,
	pub scores_command: &'static str,
	pub quit_command: &'static str,

	pub help: &'static str,
	pub vote_command_help: &'static str,
	pub command_unknown: &'static str,

	pub voted_for: &'static str,
	pub voted_blank: &'static str,
	pub voted_null: &'static str,
	pub already_voted: &'static str,

	pub scoreboard_title: &'static str,
	pub scoreboard_blank: &'static str,
	pub scoreboard_null: &'static str,

	pub voters_title: &'static str,

	pub quit: &'static str,
}