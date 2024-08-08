use candid::Nat;

use crate::provider::github::api::get_merged_details::User;

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
