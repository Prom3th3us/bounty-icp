use candid::Nat;

use crate::bounty::cmd::{
    db::Db,
    model::{Amount, CommentId, DepositLink, IssueId, IssuePk, OrgId, RepoId, Time, UserId},
};

use super::api::*;

#[cfg(test)]
pub fn issue_pk_sanity(
    db: &Db,
    issue_pk: &IssuePk,
    expected_result: &DepositLink,
    expected_amount: &Amount,
) {
    assert!(db.find(&issue_pk).is_some());

    assert_eq!(db.total_bounty_count_for(&issue_pk), Nat::from(1 as u64));
    assert_eq!(
        db.total_bounty_amount_for(&issue_pk),
        expected_amount.clone()
    );

    let db_deposit_link = db.find(&issue_pk).unwrap().deposit_link();

    assert_eq!(expected_result, db_deposit_link)
}

pub struct BountyFixture {
    pub user_id: UserId,
    pub org_id: OrgId,
    pub comment_id: CommentId,
    pub issue_id: IssueId,
    pub repo_id: RepoId,
    pub amount: Amount,
    pub now: Time,
}

impl BountyFixture {
    pub fn new(id: u64) -> Self {
        let user_id = UserId::new(format!("user_id_{id}"));
        let org_id = OrgId::new(format!("org_id_{id}"));
        let comment_id = CommentId::new(format!("comment_id_{id}"));
        let issue_id = IssueId::new(format!("issue_id_{id}"));
        let repo_id = RepoId::new(format!("repo_id_{id}"));

        let amount = Amount::new(Nat::from(100 as u64));
        let now = Time::new(20240808 as u64);

        Self {
            user_id,
            org_id,
            comment_id,
            issue_id,
            repo_id,
            amount,
            now,
        }
    }
    pub fn new_with(id: u64, org_id: &OrgId, repo_id: &RepoId) -> Self {
        let user_id = UserId::new(format!("user_id_{id}"));
        let comment_id = CommentId::new(format!("comment_id_{id}"));
        let issue_id = IssueId::new(format!("issue_id_{id}"));
        let amount = Amount::new(Nat::from(100 as u64));
        let now = Time::new(20240808 as u64);

        Self {
            user_id,
            org_id: org_id.clone(),
            comment_id,
            issue_id,
            repo_id: repo_id.clone(),
            amount,
            now,
        }
    }

    pub fn issue_pk(&self) -> IssuePk {
        IssuePk::new(
            self.org_id.clone(),
            self.repo_id.clone(),
            self.issue_id.clone(),
        )
    }

    pub fn call_bounty(&self, db: &mut Db) -> Result<Success, Failure> {
        bounty(
            db,
            self.user_id.clone(),
            self.org_id.clone(),
            self.comment_id.clone(),
            self.issue_id.clone(),
            self.repo_id.clone(),
            self.amount.clone(),
            self.now.clone(),
        )
    }
}
