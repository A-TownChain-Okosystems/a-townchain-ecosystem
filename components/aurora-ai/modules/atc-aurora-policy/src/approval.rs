#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalRequirement {
    None,
    ExplicitUser,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Approval {
    pub requirement: ApprovalRequirement,
    pub granted: bool,
}

impl Approval {
    pub fn not_required() -> Self {
        Self { requirement: ApprovalRequirement::None, granted: true }
    }

    pub fn explicit_user(granted: bool) -> Self {
        Self { requirement: ApprovalRequirement::ExplicitUser, granted }
    }

    pub fn is_satisfied(&self) -> bool {
        matches!(self.requirement, ApprovalRequirement::None) || self.granted
    }
}
