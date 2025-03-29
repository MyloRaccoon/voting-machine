use crate::{domain::{AttendenceSheet, Scoreboard, VoteOutcome}, storage::Storage, use_cases::{VoteForm, VotingController}};

use super::lexicon::Lexicon;

pub async fn handle_line<Store: Storage>(line: &str, controller: &mut VotingController<Store>, lexicon: &Lexicon) -> anyhow::Result<String> {
	let res: String;
    match line.split_whitespace().next().unwrap_or("") {
        "vote" => {
            let mut ite = line.split_whitespace();
            ite.next();
            match ite.next() {
                Some(voter_str) => {
                    let voter_string = voter_str.to_string();
                    let candidate_string = match ite.next() {
                        Some(candidate_str) => candidate_str.to_string(),
                        None => String::new(),
                    };
                    let vote_form = VoteForm{ voter: voter_string, candidate: candidate_string };
                        res = get_vote_outcome(controller.vote(vote_form).await?, lexicon);
                    },
                None => res = lexicon.vote_command_help.to_string(),
            }
        }
        "voters" => res = show_attendence_sheet(controller.get_voting_machine().await?.get_voters(), lexicon),
        "scores" => res = show_scoreboard(controller.get_voting_machine().await?.get_scoreboard(), lexicon),
        "q" => res = lexicon.quit.to_string(),
        "" => res = lexicon.help.to_string(),
        _ => res = format!("\"{}\" : {}...", line, lexicon.command_unknown),
    }
    Ok(res)
}

fn get_vote_outcome(outcome: VoteOutcome, lexicon: &Lexicon) -> String {
    match outcome {
        VoteOutcome::AcceptedVote(voter, candidate) => format!("{}{}{} !", voter.0, lexicon.voted_for, candidate.0),
        VoteOutcome::BlankVote(voter) => format!("{}{}", voter.0, lexicon.voted_blank),
        VoteOutcome::InvalidVote(voter) => format!("{}{}", voter.0, lexicon.voted_null),
        VoteOutcome::HasAlreadyVoted(voter) =>  format!("{}{}", voter.0, lexicon.already_voted),
    }
}


fn show_scoreboard(scoreboard: &Scoreboard, lexicon: &Lexicon) -> String {
    let mut res = String::from(lexicon.scoreboard_title);
    for item in scoreboard.scores.clone() {
        res += &format!(" - {} : {}\n", item.0.0, item.1.0);
    }
    res += "--\n";
    res += &format!("{}{}\n", lexicon.scoreboard_blank, scoreboard.blank_score.0);
    res += &format!("{}{}\n", lexicon.scoreboard_null, scoreboard.invalid_score.0);
    res
}

fn show_attendence_sheet(attendence_sheet: &AttendenceSheet, lexicon: &Lexicon) -> String {
    let mut res = String::from(lexicon.voters_title);
    for voter in attendence_sheet.0.clone() {
        res += &format!(" - {}\n", voter.0);
    }
    res
}