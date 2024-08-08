use std::{borrow::BorrowMut, collections::HashMap};

use super::model::*;

#[derive(Debug, Clone)]
pub struct Bounty {
    amount: Amount,
    created_at: Time,
    comment_id: CommentId,
    deposit_link: DepositLink,
    issue_pk: IssuePk,
}

impl Bounty {
    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn created_at(&self) -> &Time {
        &self.created_at
    }

    pub fn comment_id(&self) -> &CommentId {
        &self.comment_id
    }

    pub fn deposit_link(&self) -> &DepositLink {
        &self.deposit_link
    }

    pub fn issue_pk(&self) -> &IssuePk {
        &self.issue_pk
    }

    pub fn new(
        amount: Amount,
        created_at: Time,
        comment_id: CommentId,
        deposit_link: DepositLink,
        issue_pk: IssuePk,
    ) -> Self {
        Bounty {
            amount,
            created_at,
            comment_id,
            deposit_link,
            issue_pk,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Db {
    // FIXME! Make key &IssuePk
    bounties: HashMap<IssuePk, Bounty>,
}

impl Db {
    #[cfg(test)]
    pub fn all(&self) -> &HashMap<IssuePk, Bounty> {
        &self.bounties
    }

    pub fn new() -> Self {
        Db {
            bounties: HashMap::new(),
        }
    }

    pub fn insert(&mut self, bounty: Bounty) {
        self.bounties.insert(bounty.issue_pk().clone(), bounty);
    }

    pub fn find(&self, issue_pk: &IssuePk) -> Option<&Bounty> {
        self.bounties.get(issue_pk)
    }
}
