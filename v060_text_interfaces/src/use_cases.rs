use serde::Deserialize;

use crate::{domain::{BallotPaper, VoteOutcome, VotingMachine}, storage::Storage};

#[derive(Deserialize, Clone)]
pub struct VoteForm {
	pub voter: String,
	pub candidate: String,
}

pub struct VotingController<Store> {
	store: Store,
}

impl<Store: Storage> VotingController<Store> {
	
	pub fn new(store: Store) -> Self {
		Self { store }
	}

	pub async fn vote(&mut self, vote_form: VoteForm) -> anyhow::Result<VoteOutcome> {
		let mut machine = self.get_voting_machine().await?;
		let ballot_paper = BallotPaper::from(vote_form);
		let vote_outcome = machine.vote(ballot_paper);
		self.store.put_voting_machine(machine.clone()).await?;
		Ok(vote_outcome)
	}

	pub async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
		self.store.get_voting_machine().await
	}
}

#[cfg(test)]
mod tests {
    use crate::storages::memory::MemoryStore;
    use crate::domain::{Candidate, Voter, Score};
    use super::*;

    #[tokio::test]
    async fn accepted_vote_controller() {
    	let candidate = Candidate("Godot".to_string());
        let candidates = vec![candidate.clone(), Candidate("Unreal".to_string()), Candidate("Unity".to_string())];
    	let machine = VotingMachine::new(candidates);
    	let store = MemoryStore::new(machine).await.expect("couldn't create store");
        let mut controller = VotingController::new(store);
        let form = VoteForm{ voter: "Jean".to_string(), candidate: "Godot".to_string() };

        assert_eq!(controller.vote(form).await.expect("couldn't vote"), VoteOutcome::AcceptedVote(Voter("Jean".to_string()), candidate.clone()));
    	
    	let new_machine = controller.get_voting_machine().await.expect("couldn't get voting machine");
    	let scoreboard = new_machine.get_scoreboard();
    	assert_eq!(*scoreboard.scores.get(&candidate).unwrap(), Score(1));
    }

    #[tokio::test]
    async fn has_already_voted_controller() {
    	let voter = "CoolVoter".to_string();
    	let candidate = "Godot".to_string();
        let candidates = vec![Candidate(candidate.clone()), Candidate("Unreal".to_string()), Candidate("Unity".to_string())];
    	let machine = VotingMachine::new(candidates);
    	let store = MemoryStore::new(machine).await.expect("couldn't create store");
    	let mut controller = VotingController::new(store);

    	let form = VoteForm{ voter: voter.clone(), candidate: candidate.clone() };

    	controller.vote(form.clone()).await.expect("couldn't vote");

    	assert_eq!(controller.vote(form).await.expect("couldn't vote 2"), VoteOutcome::HasAlreadyVoted(Voter(voter.clone())));
    	let new_machine = controller.get_voting_machine().await.expect("couldn't get voting machine");
    	let scoreboard = new_machine.get_scoreboard();
    	assert_eq!(*scoreboard.scores.get(&Candidate(candidate)).unwrap(), Score(1));
    }

    #[tokio::test]
    async fn blank_vote_controller() {
    	let voter = "CoolVoter".to_string();
    	let candidates = vec![Candidate("Godot".to_string()), Candidate("Unreal".to_string()), Candidate("Unity".to_string())];
    	let machine = VotingMachine::new(candidates);
    	let store = MemoryStore::new(machine).await.expect("couldn't create store");
    	let mut controller = VotingController::new(store);

    	let form = VoteForm{
    		voter: voter.clone(),
    		candidate: String::new(),
    	};

    	assert_eq!(controller.vote(form).await.expect("couldn't vote"), VoteOutcome::BlankVote(Voter(voter)));
    	let new_machine = controller.get_voting_machine().await.expect("couldn't get voting machine");
    	let scoreboard = new_machine.get_scoreboard();
    	assert_eq!(scoreboard.blank_score, Score(1));
    }

    #[tokio::test]
    async fn invalid_vote_controller() {
    	let voter = "CoolVoter".to_string();
    	let candidates = vec![Candidate("Godot".to_string()), Candidate("Unreal".to_string()), Candidate("Unity".to_string())];
    	let machine = VotingMachine::new(candidates);
    	let store = MemoryStore::new(machine).await.expect("couldn't create store");
    	let mut controller = VotingController::new(store);

    	let form = VoteForm{
    		voter: voter.clone(),
    		candidate: "InvalidCandidate".to_string(),
    	};

    	assert_eq!(controller.vote(form).await.expect("couldn't vote"), VoteOutcome::InvalidVote(Voter(voter)));
    	let new_machine = controller.get_voting_machine().await.expect("couldn't get voting machine");
    	let scoreboard = new_machine.get_scoreboard();
    	assert_eq!(scoreboard.invalid_score, Score(1));
    }
}