use async_trait::async_trait;

use crate::{domain::VotingMachine, storage::Storage};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryStore {
	voting_machine: VotingMachine,
}

#[async_trait]
impl Storage for MemoryStore {
    async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
        let voting_machine: VotingMachine = machine;
        Ok(Self { voting_machine })
    }

    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        Ok(self.voting_machine.clone())
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
        self.voting_machine = machine;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::storage::Storage;
    use crate::domain::{BallotPaper, Candidate, Voter, VotingMachine};
    use super::MemoryStore;


	#[tokio::test]
	async fn test_get_put_machine_in_memory() {
		let candidates = vec![Candidate("Godot".to_string()), Candidate("Unity".to_string())];
		let mut voting_machine = VotingMachine::new(candidates);
		let mut memory = MemoryStore::new(voting_machine.clone()).await.expect("MemoryStore::new returned Err");

		let ballot_paper = BallotPaper{
			voter: Voter("Jean".to_string()),
			candidate: Some(Candidate("Godot".to_string())),
		};

		voting_machine.vote(ballot_paper);

		memory.put_voting_machine(voting_machine.clone()).await.expect("put_voting_machine returned Err");

		assert_eq!(voting_machine, memory.get_voting_machine().await.expect("get_voting_machine returned Err"));
	}
}
