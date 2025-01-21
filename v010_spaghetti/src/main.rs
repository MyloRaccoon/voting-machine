use anyhow::Error;
use std::collections::HashMap;
use tokio::io::{self, AsyncBufReadExt, BufReader};

fn print_commands() {
    println!("Commands :");
    println!(" - vote : to vote");
    println!(" - voters : see all voters");
    println!(" - scores : see scores");
    println!(" - q : quit");
}

fn print_voters(voters: &mut HashMap<String, String>) {
    println!("Voters : ");
    for voter in voters.keys() {
        println!(" - {} voted for {}", voter, voters.get(voter).unwrap());
    }
}

fn print_scores(scores: &mut HashMap<String, u32>) {
    println!("Candidates for best Game Engine :");

    for candidate in scores.keys() {
        println!(" - {} : {}", candidate, scores.get(candidate).unwrap())
    }
}

fn process_voting(
    voters: &mut HashMap<String, String>,
    voter: String,
    candidate: String,
    scores: &mut HashMap<String, u32>,
) {
    if voters.contains_key(&voter) {
        println!("Voter \"{}\" already voted !", voter);
        return;
    }

    if scores.contains_key(&candidate) {
        println!("{} vote for {} !", voter, candidate);
        scores.insert(candidate, scores.get(&candidate).unwrap() + 1);
        voters.insert(voter, candidate);
    } else if candidate == String::from("") {
        println!("{} voted white !", voter);
        voters.insert(voter, String::from("white"));
    } else {
        println!(
            "\"{}\" is not a candidate, {} voted null !",
            candidate, voter
        );
        voters.insert(voter, String::from("null"));
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut voters: HashMap<String, String> = HashMap::new();
    let mut condidates = vec![
        String::from("Godot"),
        String::from("Unity"),
        String::from("Unreal Engine"),
        String::from("Game Maker Studio"),
    ];
    let mut scores: HashMap<String, u32> = HashMap::new();

    for candidate in &condidates {
        scores.insert(*candidate, 0);
    }

    println!(" ~ Welcome ~");
    print_commands();
    println!("Please type a command :");

    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(input) = lines.next_line().await? {
        match input.trim_end() {
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
