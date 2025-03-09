use crate::interfaces::lexicon::Lexicon;

impl Lexicon {
	pub const fn english() -> Self {
		Self {
			welcome: " ~ Welcome ~",
			please: "Please type a command :",

			vote_command: "vote",
			voters_command: "voters",
			scores_command: "scores",
			quit_command: "quit",

			help: "
Commands :\n
 - vote <Voter> [<Candidate>] : to vote\n
 - voters : see all voters\n
 - scores : see scores\n
 - q : quit\n
			",
			vote_command_help: "Command \"vote\" gets 1 to 2 args <voter> [<candidate>]",
			command_unknown: "Command not known",

			voted_for: " voted for ",
			voted_blank: " voted blank ! ",
			voted_null: " voted null !",
			already_voted: " already voted !",

			scoreboard_title: "Scoreboard :\n",
			scoreboard_blank: " - Blank : ",
			scoreboard_null: " - Null : ",

			voters_title: "Voters :\n",

			quit: "Goodbye !",
		}
	}
}