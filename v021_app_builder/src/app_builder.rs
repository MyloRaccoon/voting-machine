use std::collections::BTreeMap;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::configuration::Configuration;

fn print_commands() {
    println!("Commands :");
    println!(" - vote : to vote");
    println!(" - voters : see all voters");
    println!(" - scores : see scores");
    println!(" - q : quit");
}

fn print_voters(voters: &mut BTreeMap<String, String>) {
    println!("Voters : ");
    for (voter, candidate) in voters {
        println!(" - {voter} voted for {candidate}");
    }
}

fn print_scores(scores: &mut BTreeMap<String, u32>) {
    println!("Candidates for best Game Engine :");
    for (candidate, score) in scores {
        println!(" - {candidate} : {score} votes");
    }
}

fn process_voting(
    voters: &mut BTreeMap<String, String>,
    voter: String,
    candidate: String,
    scores: &mut BTreeMap<String, u32>,
) {
    if voters.contains_key(&voter) {
        println!("Voter \"{}\" already voted !", voter);
        return;
    }

    if candidate == *"" {
        println!("{} voted white !", voter);
        voters.insert(voter, String::from("White"));
        scores.insert(
            "White".to_string(),
            scores.get(&"White".to_string()).unwrap() + 1,
        );
    } else if !scores.contains_key(&candidate)
        || candidate == *"Null"
        || candidate == *"White"
    {
        println!(
            "\"{}\" is not a candidate, {} voted null !",
            candidate, voter
        );
        voters.insert(voter, String::from("Null"));
        scores.insert(
            "Null".to_string(),
            scores.get(&"Null".to_string()).unwrap() + 1,
        );
    } else {
        println!("{} vote for {} !", voter, candidate);
        scores.insert(candidate.clone(), scores.get(&candidate).unwrap() + 1);
        voters.insert(voter, candidate.clone());
    }
}


pub async fn run_app(conf: Configuration) -> anyhow::Result<()> {
	let mut voters: BTreeMap<String, String> = BTreeMap::new();
    let mut scores: BTreeMap<String, u32> = BTreeMap::new();

    for candidate in conf.candidates {
        scores.insert(candidate.clone(), 0);
    }

    println!(" ~ Welcome ~");
    print_commands();
    println!("Please type a command :");

    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(input) = lines.next_line().await? {
        match input.split_whitespace().next().unwrap_or("") {
            "vote" => {
                let mut ite = input.split_whitespace();
                ite.next();
                let voter = ite.next().unwrap_or("");
                if !voter.is_empty() {
                    let candidate = ite.next().unwrap_or("");
                    process_voting(
                        &mut voters,
                        voter.to_string(),
                        candidate.to_string(),
                        &mut scores,
                    );
                } else {
                    println!("Command \"vote\" gets 2 args [voter] [candidate]");
                }
            }
            "voters" => print_voters(&mut voters),
            "scores" => print_scores(&mut scores),
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