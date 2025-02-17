use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Debug, Ord, PartialEq, Eq, PartialOrd, Clone)]
pub struct Voter(pub String);

#[derive(Debug, Ord, PartialEq, Eq, PartialOrd, Clone)]
pub struct Candidate(pub String);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Score(pub usize);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AttendenceSheet(pub Set<Voter>);

impl AttendenceSheet {
    pub fn print(&self) -> String {
        let mut res = String::from("Voters : \n");
        for voter in self.0.clone() {
            res += &format!(" - {}\n", voter.0);
        }
        res
    }

    pub fn contains(&self, voter: &Voter) -> bool {
        self.0.contains(voter)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scoreboard {
    pub scores: Map<Candidate, Score>,
    pub blank_score: Score,
    pub invalid_score: Score,
}

impl Scoreboard {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let mut scores: Map<Candidate, Score> = Map::new();
        for candidate in candidates {
            scores.insert(candidate, Score(0));
        }
        let blank_score = Score(0);
        let invalid_score = Score(0);
        Self { scores, blank_score, invalid_score }
    }

    pub fn print(&self) -> String {
        let mut res = String::from("Scoreboard :\n");
        for item in self.scores.clone() {
            res += &format!(" - {} : {}\n", item.0.0, item.1.0);
        }
        res += "--\n";
        res += &format!(" - Blank : {}\n", self.blank_score.0);
        res += &format!(" - Null : {}\n", self.invalid_score.0);
        res
    }
}

#[derive(Clone)]
pub struct BallotPaper {
    pub voter: Voter,
    pub candidate: Option<Candidate>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VoteOutcome {
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VotingMachine {
    voters: AttendenceSheet,
    scoreboard: Scoreboard,
}

impl VotingMachine {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let voters = AttendenceSheet(Set::<Voter>::new());
        let scoreboard = Scoreboard::new(candidates);
        Self { voters, scoreboard }
    }

    pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
        let voter = ballot_paper.voter;

        if self.voters.contains(&voter) {
            return VoteOutcome::HasAlreadyVoted(voter);
        }
        match ballot_paper.candidate {
            Some(candidate) => {
                self.voters.0.insert(voter.clone());
                match self.scoreboard.scores.get_mut(&candidate) {
                    Some(score) => {
                        score.0 += 1;
                        VoteOutcome::AcceptedVote(voter, candidate)
                    },
                    None => {
                        self.scoreboard.invalid_score.0 += 1;
                        VoteOutcome::InvalidVote(voter)
                    },
                }
            },
            None => {
                self.scoreboard.blank_score.0 += 1;
                self.voters.0.insert(voter.clone());
                VoteOutcome::BlankVote(voter.clone())
            },
        }
    }

    pub fn get_scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    pub fn get_voters(&self) -> &AttendenceSheet {
        &self.voters
    }
}


#[cfg(test)]
mod tests {
    use super::{BallotPaper, Candidate, Voter, VotingMachine, VoteOutcome, Score};

    #[test]
    fn accepted_vote() {
        let candidates = vec![Candidate("Godot".to_string()), Candidate("Unreal".to_string()), Candidate("Unity".to_string())];
        let mut voting_machine = VotingMachine::new(candidates);
        
        let candidate = Candidate("Godot".to_string());
        let voter = Voter("Jean".to_string());
        
        let ballot_paper = BallotPaper{
            voter: voter.clone(),
            candidate: Some(candidate.clone()),
        };

        assert_eq!(voting_machine.vote(ballot_paper), VoteOutcome::AcceptedVote(voter, candidate.clone()));
        assert_eq!(*voting_machine.get_scoreboard().scores.get(&candidate).unwrap(), Score(1));
    }

    #[test]
    fn has_already_voted() {
        let candidates = vec![Candidate("Godot".to_string()), Candidate("Unreal".to_string()), Candidate("Unity".to_string())];
        let mut voting_machine = VotingMachine::new(candidates);
        let candidate = Candidate("Godot".to_string());
        let voter = Voter("Jean".to_string());
        let ballot_paper = BallotPaper{
            voter: voter.clone(),
            candidate: Some(candidate.clone()),
        };
        voting_machine.vote(ballot_paper.clone());
        assert_eq!(voting_machine.vote(ballot_paper), VoteOutcome::HasAlreadyVoted(voter));
        assert_eq!(*voting_machine.get_scoreboard().scores.get(&candidate).unwrap(), Score(1));
    }

    #[test]
    fn blank_vote() {
        let candidates = vec![Candidate("Godot".to_string()), Candidate("Unreal".to_string()), Candidate("Unity".to_string())];
        let mut voting_machine = VotingMachine::new(candidates);

        let voter = Voter("Jean".to_string());

        let ballot_paper = BallotPaper{
            voter: voter.clone(),
            candidate: None,
        };

        assert_eq!(voting_machine.vote(ballot_paper), VoteOutcome::BlankVote(voter));
        assert_eq!(voting_machine.get_scoreboard().blank_score, Score(1));
    }

    #[test]
    fn invalid_vote() {
        let candidates = vec![Candidate("Godot".to_string()), Candidate("Unreal".to_string()), Candidate("Unity".to_string())];
        let mut voting_machine = VotingMachine::new(candidates);
        let voter = Voter("Jean".to_string());
        let ballot_paper = BallotPaper{
            voter: voter.clone(),
            candidate: Some(Candidate("NotInListCandidate".to_string())),
        };

        assert_eq!(voting_machine.vote(ballot_paper), VoteOutcome::InvalidVote(voter));
        assert_eq!(voting_machine.get_scoreboard().invalid_score, Score(1));
    }
}