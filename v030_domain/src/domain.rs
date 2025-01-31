use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;


pub struct Voter(pub String);

#[derive(Debug, Ord, PartialEq, Eq, PartialOrd)]
pub struct Candidate(pub String);

pub struct Score(pub usize);

pub struct AttendenceSheet(pub Set<Voter>);

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
        if self.voters.contains(ballot_paper.candidate) {
            VoteOutcome::HasAlreadyVoted(ballot_paper.candidate)
        }
        match ballot_paper.candidate {
            Some(candidate) => {
                match self.scoreboard.scores.get(&candidate) {
                    Some(score) => {
                        score.0 += 1;
                        VoteOutcome::AcceptedVote(ballot_paper.voter, candidate)
                    },
                    None => {
                        self.scoreboard.blank_score.0 += 1;
                        VoteOutcome::BlankVote(ballot_paper.voter)
                    },
                }
            },
            None => {
                self.scoreboard.invalid_score.0 += 1;
                VoteOutcome::InvalidVote(ballot_paper.voter)
            },
        }
    }
}
