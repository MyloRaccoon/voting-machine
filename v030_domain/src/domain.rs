use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Debug, Ord, PartialEq, Eq, PartialOrd, Clone)]
pub struct Voter(pub String);

#[derive(Debug, Ord, PartialEq, Eq, PartialOrd, Clone)]
pub struct Candidate(pub String);

#[derive(Clone)]
pub struct Score(pub usize);

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
        let mut res = false;
        for c_voter in self.0.clone() {
            if c_voter.0 == voter.0 {
                res = true;
                break;
            }
        }
        return res
    }
}

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

pub struct BallotPaper {
    pub voter: Voter,
    pub candidate: Option<Candidate>,
}

pub enum VoteOutcome {
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

pub struct VotingMachine {
    pub voters: AttendenceSheet,
    pub scoreboard: Scoreboard,
}

impl VotingMachine {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let voters = AttendenceSheet(Set::<Voter>::new());
        let scoreboard = Scoreboard::new(candidates);
        Self { voters, scoreboard }
    }

    pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
        let voter = ballot_paper.voter;

        match ballot_paper.candidate {
            Some(candidate) => {
                if self.voters.contains(&voter) {
                    return VoteOutcome::HasAlreadyVoted(voter);
                }
                self.voters.0.insert(voter.clone());
                match self.scoreboard.scores.get_mut(&candidate) {
                    Some(score) => {
                        score.0 += 1;
                        return VoteOutcome::AcceptedVote(voter, candidate);
                    },
                    None => {
                        self.scoreboard.invalid_score.0 += 1;
                        return VoteOutcome::InvalidVote(voter);
                    },
                }
            },
            None => {
                self.scoreboard.blank_score.0 += 1;
                self.voters.0.insert(voter.clone());
                return VoteOutcome::BlankVote(voter.clone());
            },
        }
    }
}
