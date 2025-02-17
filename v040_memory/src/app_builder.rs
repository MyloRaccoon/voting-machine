use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::{configuration::Configuration, domain::{BallotPaper, Candidate, VoteOutcome, Voter, VotingMachine}, storage::Storage, storages::memory::{MemoryStore}};

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

pub async fn run_app(conf: Configuration) -> anyhow::Result<()> {
    let mut memory = MemoryStore::new(create_voting_machine(&conf)).await?;

    println!(" ~ Welcome ~");
    print_commands();
    println!("Please type a command :");

    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(input) = lines.next_line().await? {
        let mut voting_machine = memory.get_voting_machine().await?;
        match input.split_whitespace().next().unwrap_or("") {
            "vote" => {
                let mut ite = input.split_whitespace();
                ite.next();
                match ite.next() {
                    Some(voter_str) => {
                        let voter = Voter(voter_str.to_string());
                        let candidate = ite.next().map(|candidate_str| Candidate(candidate_str.to_string()));
                        let ballot_paper = BallotPaper{
                            voter,
                            candidate,
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
            "voters" => println!("{}", voting_machine.get_voters().print()),
            "scores" => println!("{}", voting_machine.get_scoreboard().print()),
            "q" => {
                println!("Goodbye !");
                break;
            }
            "" => print_commands(),
            _ => println!("Command \"{}\" not known...", input),
        }
        memory.put_voting_machine(voting_machine).await?;
        println!("\nPlease type a command :");
    }

    Ok(())
}