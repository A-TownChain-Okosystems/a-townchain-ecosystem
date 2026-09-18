use std::collections::BTreeMap;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ProposalStatus{Pending,Active,Passed,Rejected,Queued,Executed,Cancelled}

#[derive(Debug,Clone)]
pub struct Proposal{pub id:u64,pub title:String,pub description:String,pub proposer:String,pub status:ProposalStatus,pub for_votes:u64,pub against_votes:u64,pub abstain_votes:u64,pub start_block:u64,pub end_block:u64}

impl Proposal{
 pub fn new(id:u64,title:&str,desc:&str,proposer:&str,start:u64,end:u64)->Self{assert!(end>start,"proposal voting window must be positive");Self{id,title:title.into(),description:desc.into(),proposer:proposer.into(),status:ProposalStatus::Pending,for_votes:0,against_votes:0,abstain_votes:0,start_block:start,end_block:end}}
 pub fn activate(&mut self,current_block:u64)->Result<(),String>{if self.status!=ProposalStatus::Pending{return Err("proposal is not pending".into())}if current_block<self.start_block{return Err("voting window has not started".into())}self.status=ProposalStatus::Active;Ok(())}
 pub fn tally(&mut self,quorum:u64,approval_bps:u16)->Result<(),String>{if self.status!=ProposalStatus::Active{return Err("proposal is not active".into())}if approval_bps>10_000{return Err("approval threshold exceeds 10000 bps".into())}let participation=self.for_votes.saturating_add(self.against_votes).saturating_add(self.abstain_votes);if participation<quorum{self.status=ProposalStatus::Rejected;return Ok(())}let decided=self.for_votes.saturating_add(self.against_votes);if decided==0{self.status=ProposalStatus::Rejected;return Ok(())}if self.for_votes.saturating_mul(10_000)>=decided.saturating_mul(approval_bps as u64){self.status=ProposalStatus::Passed}else{self.status=ProposalStatus::Rejected}Ok(())}
 pub fn cancel(&mut self)->Result<(),String>{if matches!(self.status,ProposalStatus::Executed|ProposalStatus::Cancelled){return Err("proposal cannot be cancelled".into())}self.status=ProposalStatus::Cancelled;Ok(())}
}
