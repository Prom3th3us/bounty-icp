use crate::bounty::cmd::db::*;
use crate::bounty::cmd::model::*;

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
    let issue_pk = IssuePk::new(org_id, repo_id, issue_id);

    match db.find(&issue_pk) {
        Some(bounty) => Ok(bounty.deposit_link().clone()),
        None => {
            let created_at = now;
            let deposit_link = DepositLink::new(format!("{:?}", issue_pk));

            let bounty = Bounty::new(
                amount,
                created_at,
                comment_id,
                deposit_link.clone(),
                issue_pk,
                user_id,
            );
            db.insert(bounty);
            Ok(deposit_link)
        }
    }
}

#[cfg(test)]

mod test_bounty {
    use super::*;
    use crate::bounty::cmd::{
        bounty::fixture::{issue_pk_sanity, BountyFixture},
        db::Db,
    };
    use candid::Nat;

    /*
    1: On "/bounty Amount" for non-existing issue:
    Verify a bounty(amount) is created
    Verify the bounty is attached to the issue
    Verify a deposit link is returned
    */
    #[test]
    fn test_1() {
        let mut db: Db = Db::new();

        let fixture = BountyFixture::new(1);

        let issue_pk = fixture.issue_pk();
        let result = fixture.call_bounty(&mut db);

        assert!(result.is_ok());

        issue_pk_sanity(&db, &issue_pk, &result.unwrap(), &fixture.amount);
    }

    /*
        3: On multiple "/bounty Amount" for non-existing issue:
            Verify a bounty1(amount) is created
            Verify the bounty1 is attached to the issue
            Verify a deposit link1 is returned
            Verify a bounty2(amount) is created
            Verify the bounty2 is attached to the issue
            Verify a deposit link2 is returned
            Verify the issue has two bounties attached
    */
    #[test]
    fn test_3() {
        todo!()
    }

    /*
        4: Guarantee issue_pk is unique across diff repos over the same org:
            Verify a bounty1(amount, org_1, repo_1, issue_id_1) is created
            Verify the bounty1 is attached to the issue_pk1 *
            Verify a deposit link1 is returned *
            Verify a bounty2(amount, org_1, repo_2, issue_id_1) is created
            Verify the bounty2 is attached to the issue_pk2 *
            Verify a deposit link2 is returned *
            Verify a deposit link1 and deposit link2 are different

    */
    #[test]
    fn test_4() {
        let mut db = Db::new();

        let org_id = OrgId::new(String::from("org_id"));
        let repo_id_1 = RepoId::new(String::from("repo_id_1"));
        let fixture_1 = BountyFixture::new_with(1, &org_id, &repo_id_1);

        let issue_pk_1 = fixture_1.issue_pk();
        let result_1 = fixture_1.call_bounty(&mut db);

        assert!(result_1.is_ok());

        issue_pk_sanity(
            &db,
            &issue_pk_1,
            &result_1.clone().unwrap(),
            &fixture_1.amount,
        );

        let repo_id_2 = RepoId::new(String::from("repo_id_2"));
        let fixture_2 = BountyFixture::new_with(1, &org_id, &repo_id_2);

        let issue_pk_2 = fixture_2.issue_pk();
        let result_2 = fixture_2.call_bounty(&mut db);

        assert!(result_2.is_ok());

        issue_pk_sanity(
            &db,
            &issue_pk_2,
            &result_2.clone().unwrap(),
            &fixture_2.amount,
        );

        assert_ne!(result_1, result_2);

        assert_eq!(
            db.total_bounty_amount(),
            fixture_1.amount + fixture_2.amount
        );
        assert_eq!(db.total_bounty_count(), Nat::from(2 as u64));
    }

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

    #[test]
    fn test_5() {
        let mut db = Db::new();

        let org_id_1 = OrgId::new(String::from("org_id_1"));
        let repo_id_1 = RepoId::new(String::from("repo_id_1"));
        let fixture_1 = BountyFixture::new_with(1, &org_id_1, &repo_id_1);

        let issue_pk_1 = fixture_1.issue_pk();
        let result_1 = fixture_1.call_bounty(&mut db);

        assert!(result_1.is_ok());

        issue_pk_sanity(
            &db,
            &issue_pk_1,
            &result_1.clone().unwrap(),
            &fixture_1.amount,
        );

        let org_id_2 = OrgId::new(String::from("org_id_2"));
        let repo_id_2 = RepoId::new(String::from("repo_id_2"));
        let fixture_2 = BountyFixture::new_with(1, &org_id_2, &repo_id_2);

        let issue_pk_2 = fixture_2.issue_pk();
        let result_2 = fixture_2.call_bounty(&mut db);

        assert!(result_2.is_ok());

        issue_pk_sanity(
            &db,
            &issue_pk_2,
            &result_2.clone().unwrap(),
            &fixture_2.amount,
        );

        assert_ne!(result_1, result_2);

        assert_eq!(
            db.total_bounty_amount(),
            fixture_1.amount + fixture_2.amount
        );
        assert_eq!(db.total_bounty_count(), Nat::from(2 as u64));
    }

    /*
        6: Guarantee idempotency for same comment_id on existing issue:
            Verify an issue exists
            Verify a bounty1(amount, issue_pk_1,comment_id_1) is created
            Verify the bounty1 is attached to the issue_pk_1
            Verify a deposit link1 is returned
            Verify a bounty2(amount, issue_pk_1, comment_id_1) is created
            Verify deposit link2 is returned
            Verify bounty1 and bounty2 should return the same bounty
    */
    #[test]
    fn test_6() {
        let mut db = Db::new();

        let fixture = BountyFixture::new(1);

        let issue_pk = fixture.issue_pk();
        let result_1 = fixture.call_bounty(&mut db);

        assert!(result_1.is_ok());

        issue_pk_sanity(&db, &issue_pk, &result_1.clone().unwrap(), &fixture.amount);

        let result_2 = fixture.call_bounty(&mut db);

        assert!(result_2.is_ok());

        issue_pk_sanity(&db, &issue_pk, &result_2.clone().unwrap(), &fixture.amount);

        assert_eq!(result_1, result_2);

        assert_eq!(db.total_bounty_amount(), fixture.amount);
        assert_eq!(db.total_bounty_count(), Nat::from(1 as u64));
    }
}
