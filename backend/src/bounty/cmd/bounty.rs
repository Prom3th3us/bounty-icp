use super::db::*;
use super::model::*;

pub type Failure = ();
pub type Success = DepositLink;

pub fn bounty(
    db: &mut Db,
    user_id: UserId,
    org_id: OrgId,
    comment_id: CommentId,
    issue_id: IssueId,
    repo_id: RepoId,
    amount: Amount,
    now: Time,
) -> Result<Success, Failure> {
    /*
        issue_pk = {org_id, repo_id, issue_id}
        if issue_pk not exist {
            let issue = new Issue(issue_pk)
            state.insert(issue_pk, issue)
        }
        let bounty = new Bounty(issue_pk, comment_id, amount)
        state.insert(issue_pk -> bounty)
        let deposit_link = create_deposit_link(bounty)
        return deposit_link
    */
    todo!()
}

#[cfg(test)]

mod test_bounty {
    use crate::bounty::api::state;

    use super::*;
    use candid::Nat;
    /*
    1: On "/bounty Amount" for existing issue:
    Verify an issue_pk exists
    Verify a bounty(amount) is created
    Verify the bounty is attached to the issue
    Verify a deposit link is returned
    */
    #[test]
    fn test_1() {
        let mut db = Db::new();

        let user_id = UserId::new(String::from("user_id_1"));
        let org_id = OrgId::new(String::from("org_id_1"));
        let comment_id = CommentId::new(String::from("comment_id_1"));
        let issue_id = IssueId::new(String::from("issue_id_1"));
        let repo_id = RepoId::new(String::from("repo_id_1"));
        let amount = Amount::new(Nat::from(100 as u64));
        let now = Time::new(20240808 as u64);

        let issue_pk = IssuePk::new(org_id.clone(), repo_id.clone(), issue_id.clone());

        assert!(&db.find(&issue_pk).is_none());

        let result = bounty(
            &mut db,
            user_id,
            org_id.clone(),
            comment_id,
            issue_id.clone(),
            repo_id.clone(),
            amount,
            now,
        );

        assert!(result.is_ok());
        assert!(&db.find(&issue_pk).is_some());
    }

    /*
        2: On "/bounty Amount" for non-existing issue:
            Verify an issue non-exists
            Verify the issued is created
            Verify an issue exists
            Verify a bounty(amount) is created
            Verify the bounty is attached to the issue
            Verify a deposit link is returned
    */
    fn test_2() {
        todo!()
    }

    /*
        3: On multiple "/bounty Amount" for existing issue:
            Verify an issue exists
            Verify a bounty1(amount) is created
            Verify the bounty1 is attached to the issue
            Verify a deposit link1 is returned
            Verify a bounty2(amount) is created
            Verify the bounty2 is attached to the issue
            Verify a deposit link2 is returned
            Verify the issue has two bounties attached
    */
    fn test_3() {
        todo!()
    }

    /*
        4: Guarantee issue_pk is unique across diff repos over the same org:
            Verify an issue exists
            Verify a bounty1(amount, org_1, repo_1, issue_id_1) is created
            Verify the bounty1 is attached to the issue_pk1
            Verify a deposit link1 is returned
            Verify a bounty2(amount, org_1, repo_2, issue_id_1) is created
            Verify the bounty2 is attached to the issue_pk2
            Verify a deposit link2 is returned
            Verify a deposit link1 and deposit link2 are different

    */

    /*
        5: Guarantee issue_pk is unique across diff repos over diff orgs:
            Verify an issue exists
            Verify a bounty1(amount, org_1, repo_1, issue_id_1) is created
            Verify the bounty1 is attached to the issue_pk1
            Verify a deposit link1 is returned
            Verify a bounty2(amount, org_2, repo_2, issue_id_1) is created
            Verify the bounty2 is attached to the issue_pk2
            Verify a deposit link2 is returned
            Verify a deposit link1 and deposit link2 are different


    */

    /*
        6: Guarantee idempotency for same comment_id on existing issue:
            Verify an issue exists
            Verify a bounty1(amount, issue_pk_1,comment_id_1) is created
            Verify the bounty1 is attached to the issue_pk_1
            Verify a deposit link1 is returned
            Verify creation of bounty2(amount, issue_pk_2, comment_id_1) is rejected
            Verify deposit link2 is not returned
            Verify a failure response is generated

    */
}
