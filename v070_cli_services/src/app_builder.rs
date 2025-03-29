use crate::{service::Service, configuration::{Configuration, Language}, domain::{Candidate, VotingMachine}, interfaces::lexicon::Lexicon, services::{stdio::StdioService, udp::UdpService}, storage::Storage, use_cases::VotingController};

fn create_voting_machine(conf: &Configuration) -> VotingMachine {
    let mut candidates: Vec<Candidate> = Vec::new();

    for candidate in conf.candidates.clone() {
        candidates.push(Candidate(candidate));
    }

    VotingMachine::new(candidates)
}

pub async fn dispatch_service<Store: Storage + Send + Sync>(conf: Configuration) -> Result <(), anyhow::Error> {
    match conf.service {
        crate::configuration::Service::Stdio => handle_lines::<Store, StdioService<Store>>(conf).await,
        crate::configuration::Service::Udp => handle_lines::<Store, UdpService<Store>>(conf).await,
    }
}

async fn handle_lines<Store: Storage, Serv: Service<Store>>(conf: Configuration) -> anyhow::Result<()> {
    let voting_controller = VotingController::new(Store::new(create_voting_machine(&conf)).await?);

    let lexicon = match conf.language {
        Language::En => Lexicon::english(),
        Language::Fr => Lexicon::french(),
    };

    println!("{}", lexicon.welcome);
    println!("{}", lexicon.help);
    println!("{}", lexicon.please);

    let port = conf.port.unwrap_or(8888);

    Serv::new(port, lexicon, voting_controller).serve().await
}