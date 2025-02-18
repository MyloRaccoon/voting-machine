use async_trait::async_trait;
use serde_json::from_slice;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use serde::{Serialize, Deserialize};
use crate::domain::Scoreboard;
use crate::domain::VotingMachine;
use crate::storage::Storage;
use std::collections::BTreeSet as Set;
use std::collections::BTreeMap as Map;

const FILE_PATH: &str = "machine.json";

#[derive(Serialize, Deserialize)]
pub struct ScoreboardDao {
	pub scores: Map<String, usize>,
	pub blank_score: usize,
	pub invalid_score: usize,
}

impl From<Scoreboard> for ScoreboardDao {
	fn from(scoreboard: Scoreboard) -> Self {
		let mut scores = Map::new();
		for item in scoreboard.scores {
			scores.insert(item.0.0, item.1.0);
		}
		let blank_score = scoreboard.blank_score.0;
		let invalid_score = scoreboard.invalid_score.0;
		Self { 
			scores, 
			blank_score, 
			invalid_score, 
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct VotingMachineDao {
	pub voters: Set<String>,
	pub scoreboard: ScoreboardDao,
}

impl From<VotingMachine> for VotingMachineDao {
	fn from(voting_machine: VotingMachine) -> Self {
		let mut voters = Set::new();
		for voter in voting_machine.get_voters().clone().0 {
			voters.insert(voter.0);
		}
		let scoreboard = ScoreboardDao::from(voting_machine.get_scoreboard().clone());
		Self { voters, scoreboard }
	}
}



pub struct FileStore {
	filepath: String,
}

impl FileStore {
	pub async fn create(machine: VotingMachine, filepath: &str) -> anyhow::Result<Self> {
		let filepath = filepath.to_string();
		// let mut file = File::open(filepath.clone()).await?;
		let mut file = File::create(filepath.clone()).await?;
		let machine_dao = VotingMachineDao::from(machine);
		let data = serde_json::to_string(&machine_dao)?;
		file.flush().await?;
		file.write_all(data.as_bytes()).await?;
		Ok( Self { filepath } )
	}
}

#[async_trait]
impl Storage for FileStore {
    async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
        Self::create(machine, self::FILE_PATH).await
    }

    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        let mut file = File::open(self.filepath.clone()).await?;
		let mut data_bytes = vec![];
		file.read_to_end(&mut data_bytes).await?;
		let machine_dao: VotingMachineDao = from_slice(&data_bytes)?;
		Ok(VotingMachine::from(machine_dao))
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
        let mut file = File::create(self.filepath.clone()).await?;
		let machine_dao = VotingMachineDao::from(machine);
		let data = serde_json::to_string(&machine_dao)?;
		file.write_all(data.as_bytes()).await?;
		file.flush().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::Storage;
    use crate::domain::{BallotPaper, Candidate, Voter, VotingMachine};
    use super::FileStore;

	#[tokio::test]
	async fn test_get_put_machine_in_file() {
		let candidates = vec![Candidate("Godot".to_string()), Candidate("Unity".to_string())];
		let mut voting_machine = VotingMachine::new(candidates);
		let mut file = FileStore::new(voting_machine.clone()).await.expect("FileStore::new returned Err");

		let ballot_paper = BallotPaper{
			voter: Voter("Jean".to_string()),
			candidate: Some(Candidate("Godot".to_string())),
		};
		let ballot_paper2 = BallotPaper{
			voter: Voter("Gaston".to_string()),
			candidate: Some(Candidate("invalid".to_string())),
		};

		voting_machine.vote(ballot_paper);
		voting_machine.vote(ballot_paper2);

		file.put_voting_machine(voting_machine.clone()).await.expect("put_voting_machine returned Err");

		assert_eq!(voting_machine, file.get_voting_machine().await.expect("get_voting_machine returned Err"));
	}

	#[tokio::test]
	async fn test_double_file_instance() {
		let candidates = vec![Candidate("Godot".to_string()), Candidate("Unity".to_string())];
		let mut voting_machine = VotingMachine::new(candidates);
		let mut file = FileStore::new(voting_machine.clone()).await.expect("FileStore::new returned Err");
		let file2 = FileStore::new(voting_machine.clone()).await.expect("FileStore::new 2 returned Err");

		let ballot_paper = BallotPaper{
			voter: Voter("Jean".to_string()),
			candidate: Some(Candidate("Godot".to_string())),
		};

		voting_machine.vote(ballot_paper);

		file.put_voting_machine(voting_machine.clone()).await.expect("put_voting_machine returned Err");

		assert_eq!(file.get_voting_machine().await.expect("get_voting_machine returned Err"), file2.get_voting_machine().await.expect("get_voting_machine 2 returned Err"));
	}
}
