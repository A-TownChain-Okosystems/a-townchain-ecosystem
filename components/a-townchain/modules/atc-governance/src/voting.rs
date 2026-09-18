use std::collections::BTreeMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoteType {
    For,
    Against,
    Abstain,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    pub voter: String,
    pub proposal_id: u64,
    pub vote_type: VoteType,
    pub weight: u64,
}
#[derive(Default, Clone)]
pub struct VotingSystem {
    votes: BTreeMap<(String, u64), Vote>,
}
impl VotingSystem {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn cast_vote(
        &mut self,
        voter: &str,
        proposal_id: u64,
        vote_type: VoteType,
        weight: u64,
    ) -> Result<(), String> {
        if weight == 0 {
            return Err("vote weight must be non-zero".into());
        }
        let k = (voter.to_owned(), proposal_id);
        if self.votes.contains_key(&k) {
            return Err("Already voted".into());
        }
        self.votes.insert(
            k,
            Vote {
                voter: voter.into(),
                proposal_id,
                vote_type,
                weight,
            },
        );
        Ok(())
    }
    pub fn get_vote(&self, voter: &str, proposal_id: u64) -> Option<&Vote> {
        self.votes.get(&(voter.to_owned(), proposal_id))
    }
    pub fn tally(&self, id: u64) -> (u64, u64, u64) {
        self.votes
            .values()
            .filter(|v| v.proposal_id == id)
            .fold((0, 0, 0), |(f, a, x), v| match v.vote_type {
                VoteType::For => (f.saturating_add(v.weight), a, x),
                VoteType::Against => (f, a.saturating_add(v.weight), x),
                VoteType::Abstain => (f, a, x.saturating_add(v.weight)),
            })
    }
    pub fn voter_count(&self, id: u64) -> usize {
        self.votes.values().filter(|v| v.proposal_id == id).count()
    }
}
