use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::{configuration::Configuration, domain::{BallotPaper, Candidate, VoteOutcome, Voter, VotingMachine}};

fn print_commands() {
    println!("Commands :");
    println!(" - vote : to vote");
    println!(" - voters : see all voters");
    println!(" - scores : see scores");
    println!(" - q : quit");
}

// fn print_voters(voters: &mut BTreeMap<String, String>) {
//     println!("Voters : ");
//     for (voter, candidate) in voters {
//         println!(" - {voter} voted for {candidate}");
//     }
// }

// fn print_scores(scores: &mut BTreeMap<String, u32>) {
//     println!("Candidates for best Game Engine :");
//     for (candidate, score) in scores {
//         println!(" - {candidate} : {score} votes");
//     }
// }

// fn process_voting(
//     voters: &mut BTreeMap<String, String>,
//     voter: String,
//     candidate: String,
//     scores: &mut BTreeMap<String, u32>,
// ) {
//     if voters.contains_key(&voter) {
//         println!("Voter \"{}\" already voted !", voter);
//         return;
//     }

//     if candidate == String::from("") {
//         println!("{} voted white !", voter);
//         voters.insert(voter, String::from("White"));
//         scores.insert(
//             "White".to_string(),
//             scores.get(&"White".to_string()).unwrap() + 1,
//         );
//     } else if !scores.contains_key(&candidate)
//         || candidate == String::from("Null")
//         || candidate == String::from("White")
//     {
//         println!(
//             "\"{}\" is not a candidate, {} voted null !",
//             candidate, voter
//         );
//         voters.insert(voter, String::from("Null"));
//         scores.insert(
//             "Null".to_string(),
//             scores.get(&"Null".to_string()).unwrap() + 1,
//         );
//     } else {
//         println!("{} vote for {} !", voter, candidate);
//         scores.insert(candidate.clone(), scores.get(&candidate).unwrap() + 1);
//         voters.insert(voter, candidate.clone());
//     }
// }


pub async fn run_app(conf: Configuration) -> anyhow::Result<()> {
	let mut candidates: Vec<Candidate> = Vec::new();

    for candidate in conf.candidates {
        candidates.push(Candidate(candidate));
    }

    let mut voting_machine: VotingMachine = VotingMachine::new(candidates);

    println!(" ~ Welcome ~");
    print_commands();
    println!("Please type a command :");

    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(input) = lines.next_line().await? {
        match input.split_whitespace().next().unwrap_or("") {
            "vote" => {
                let mut ite = input.split_whitespace();
                ite.next();
                match ite.next() {
                    Some(voter_str) => {
                        let voter = Voter(voter_str.to_string());
                        let candidate = match ite.next() {
                            Some(candidate_str) => Some(Candidate(candidate_str.to_string())),
                            None => None,
                        };
                        let ballot_paper = BallotPaper{
                            voter: voter,
                            candidate: candidate,
                        };
                        match voting_machine.vote(ballot_paper) {
                            VoteOutcome::AcceptedVote(voter, candidate) => println!("{} voted for {} !", voter.0, candidate.0),
                            VoteOutcome::BlankVote(voter) => println!("{} voted blank !", voter.0),
                            VoteOutcome::InvalidVote(voter) => println!("{} voted null !", voter.0),
                            VoteOutcome::HasAlreadyVoted(voter) => println!("{} already voted !", voter.0),
                        }
                    },
                    None => println!("Command \"vote\" gets 1 to 2 args <voter> [<candidate>]"),
                }
            }
            "voters" => println!("{}", voting_machine.voters.print()),
            "scores" => println!("{}", voting_machine.scoreboard.print()),
            "q" => {
                println!("Goodbye !");
                break;
            }
            "" => print_commands(),
            _ => println!("Command \"{}\" not known...", input),
        }
        println!("\nPlease type a command :");
    }

    Ok(())
}