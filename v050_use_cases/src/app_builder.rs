use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::{configuration::Configuration, domain::{BallotPaper, Candidate, VoteOutcome, Voter, VotingMachine}, storage::Storage, use_cases::{VoteForm, VotingController}};

fn print_commands() {
    println!("Commands :");
    println!(" - vote : to vote");
    println!(" - voters : see all voters");
    println!(" - scores : see scores");
    println!(" - q : quit");
}

fn create_voting_machine(conf: &Configuration) -> VotingMachine {
    let mut candidates: Vec<Candidate> = Vec::new();

    for candidate in conf.candidates.clone() {
        candidates.push(Candidate(candidate));
    }

    VotingMachine::new(candidates)
}

pub async fn handle_lines<Store: Storage>(conf: Configuration) -> anyhow::Result<()> {
    let mut voting_controller = VotingController::new(Store::new(create_voting_machine(&conf)).await?);

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
                        let voter_string = voter_str.to_string();
                        let candidate_string = match ite.next() {
                            Some(candidate_str) => candidate_str.to_string(),
                            None => String::new(),
                        };
                        let vote_form = VoteForm{ voter: voter_string, candidate: candidate_string };
                            match voting_controller.vote(vote_form).await? {
                                VoteOutcome::AcceptedVote(voter, candidate) => println!("{} voted for {} !", voter.0, candidate.0),
                                VoteOutcome::BlankVote(voter) => println!("{} voted blank !", voter.0),
                                VoteOutcome::InvalidVote(voter) => println!("{} voted null !", voter.0),
                                VoteOutcome::HasAlreadyVoted(voter) => println!("{} already voted !", voter.0),
                            }
                        },
                    None => println!("Command \"vote\" gets 1 to 2 args <voter> [<candidate>]"),
                }
            }
            "voters" => println!("{}", voting_controller.get_voting_machine().await?.get_voters().print()),
            "scores" => println!("{}", voting_controller.get_voting_machine().await?.get_scoreboard().print()),
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