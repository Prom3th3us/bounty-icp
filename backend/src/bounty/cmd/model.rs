use candid::Nat;

#[derive(Debug, Clone)]
pub struct UserId {
    value: String,
}

impl UserId {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn new(value: String) -> Self {
        UserId { value }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct OrgId {
    value: String,
}

impl OrgId {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn new(value: String) -> Self {
        OrgId { value }
    }
}

#[derive(Debug, Clone)]
pub struct CommentId {
    value: String,
}

impl CommentId {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn new(value: String) -> Self {
        CommentId { value }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct IssueId {
    value: String,
}

impl IssueId {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn new(value: String) -> Self {
        IssueId { value }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct RepoId {
    value: String,
}

impl RepoId {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn new(value: String) -> Self {
        RepoId { value }
    }
}

#[derive(Debug, Clone)]
pub struct Time {
    value: u64,
}

impl Time {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn new(value: u64) -> Self {
        Time { value }
    }
}

#[derive(Debug, Clone)]
pub struct Amount {
    value: Nat,
}

impl Amount {
    pub fn value(&self) -> &Nat {
        &self.value
    }

    pub fn new(value: Nat) -> Self {
        Amount { value }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct IssuePk {
    org_id: OrgId,
    repo_id: RepoId,
    issue_id: IssueId,
}

impl IssuePk {
    pub fn org_id(&self) -> &OrgId {
        &self.org_id
    }
    pub fn repo_id(&self) -> &RepoId {
        &self.repo_id
    }
    pub fn issue_id(&self) -> &IssueId {
        &self.issue_id
    }

    pub fn new(org_id: OrgId, repo_id: RepoId, issue_id: IssueId) -> Self {
        Self {
            org_id,
            repo_id,
            issue_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DepositLink {
    value: String,
}

impl DepositLink {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn new(value: String) -> Self {
        DepositLink { value }
    }
}
