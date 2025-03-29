use crate::interfaces::lexicon::Lexicon;

impl Lexicon {
	pub const fn french() -> Self {
		Self {
			welcome: " ~ Bienvenue ~",
			please: "Veuillez entrer une commande :",

			vote_command: "voter",
			voters_command: "votants",
			scores_command: "scores",
			quit_command: "quitter",

			help: "
Commands :\n
 - vote <Votant> [<Candidat>] : pour voter\n
 - voters : voir tous les votants\n
 - scores : voir les scores\n
 - q : quitter\n
			",
			vote_command_help: "La commande \"voter\" prend 1 à 2 arguments: <Votant> [<Candidat>]",
			command_unknown: "Commande non reconnu",

			voted_for: " à voté pour ",
			voted_blank: " à voté blanc ! ",
			voted_null: " à voté nul !",
			already_voted: " à déjà voté !",

			scoreboard_title: "Tableau des scores :\n",
			scoreboard_blank: " - Blanc : ",
			scoreboard_null: " - Nul : ",

			voters_title: "Votants :\n",

			quit: "Au revoir !",
		}
	}
}