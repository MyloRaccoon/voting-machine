use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::{configuration::{Configuration, Language}, domain::{Candidate, VotingMachine}, interfaces::{cli_interface::handle_line, lexicon::{Lexicon}}, storage::Storage, use_cases::VotingController};

fn create_voting_machine(conf: &Configuration) -> VotingMachine {
    let mut candidates: Vec<Candidate> = Vec::new();

    for candidate in conf.candidates.clone() {
        candidates.push(Candidate(candidate));
    }

    VotingMachine::new(candidates)
}

pub async fn handle_lines<Store: Storage>(conf: Configuration) -> anyhow::Result<()> {
    let mut voting_controller = VotingController::new(Store::new(create_voting_machine(&conf)).await?);

    let lexicon = match conf.language {
        Language::En => Lexicon::english(),
        Language::Fr => Lexicon::french(),
    };

    println!("{}", lexicon.welcome);
    println!("{}", lexicon.help);
    println!("{}", lexicon.please);

    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(input) = lines.next_line().await? {
        let res = handle_line(&input, &mut voting_controller, &lexicon).await?;
        println!("{}", res);
        if res == lexicon.quit {
            break;
        } else {
            println!("{}", lexicon.please);
        }
    }

    Ok(())
}