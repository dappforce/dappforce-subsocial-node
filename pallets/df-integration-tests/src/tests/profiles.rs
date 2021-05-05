use super::*;
use pallet_profiles::{ProfileUpdate, Error as ProfilesError};

parameter_types! {
    pub const MaxCreationsPerPeriod: u32 = 1;
    pub const BlocksInPeriod: BlockNumber = 1;
    pub AddSocialAccountMembers: Vec<AccountId> = OffchainMembership::members();
}

impl pallet_profiles::Trait for TestRuntime {
    type Event = ();
    type AfterProfileUpdated = ProfileHistory;
    type MaxCreationsPerPeriod = MaxCreationsPerPeriod;
    type BlocksInPeriod = BlocksInPeriod;
    type FaucetsProvider = Faucets;
    type AddSocialAccountMembers = AddSocialAccountMembers;
}

impl ExtBuilder {
    pub fn build_with_one_default_social_account() -> TestExternalities {
        let mut ext = Self::build_with_faucet();
        ext.execute_with(|| { assert_ok!(_create_default_social_account()); });
        ext
    }
}
pub(crate) const PERIOD: u64 = 1;
pub(crate) const NEXT_PERIOD: u64 = 2;

fn _create_default_profile() -> DispatchResult {
    _create_profile(None, None)
}

fn _create_profile(
    origin: Option<Origin>,
    content: Option<Content>
) -> DispatchResult {
    Profiles::create_profile(
        origin.unwrap_or_else(|| Origin::signed(ACCOUNT1)),
        content.unwrap_or_else(profile_content_ipfs),
    )
}

fn _update_profile(
    origin: Option<Origin>,
    content: Option<Content>
) -> DispatchResult {
    Profiles::update_profile(
        origin.unwrap_or_else(|| Origin::signed(ACCOUNT1)),
        ProfileUpdate {
            content,
        },
    )
}

pub(crate) fn _create_default_social_account() -> DispatchResult {
    _create_social_account(None, None, None, None)
}

pub(crate) fn _create_social_account(
    origin: Option<Origin>,
    new_account: Option<AccountId>,
    referrer: Option<Option<AccountId>>,
    drip_amount: Option<Option<Balance>>
) -> DispatchResult {
    Profiles::create_social_account(
        origin.unwrap_or_else(|| Origin::signed(FAUCET1)),
        new_account.unwrap_or(ACCOUNT4),
        referrer.unwrap_or(None),
        drip_amount.unwrap_or(None)
    )
}

fn check_profile_storages(
    new_account: AccountId,
    referrer: Option<AccountId>,
    balance: Balance,
    period: u64
) {
    let social_account = Profiles::social_account_by_id(new_account).unwrap();
    assert_eq!(social_account.referrer, referrer);
    assert_eq!(Balances::free_balance(new_account), balance);

    let created_in_current_period = Profiles::created_in_current_period();
    let next_period_at = Profiles::next_period_at();

    assert_eq!(created_in_current_period, 1);
    assert_eq!(next_period_at, period);
}

// Create profile tests
// ----------------------------------------------------------------------------

#[test]
fn create_profile_should_work() {
    ExtBuilder::build().execute_with(|| {
        assert_ok!(_create_default_profile()); // AccountId 1

        let profile = Profiles::social_account_by_id(ACCOUNT1).unwrap().profile.unwrap();
        assert_eq!(profile.created.account, ACCOUNT1);
        assert!(profile.updated.is_none());
        assert_eq!(profile.content, profile_content_ipfs());

        assert!(ProfileHistory::edit_history(ACCOUNT1).is_empty());
    });
}

#[test]
fn create_profile_should_fail_when_profile_is_already_created() {
    ExtBuilder::build().execute_with(|| {
        assert_ok!(_create_default_profile());
        // AccountId 1
        assert_noop!(_create_default_profile(), ProfilesError::<TestRuntime>::ProfileAlreadyCreated);
    });
}

#[test]
fn create_profile_should_fail_when_ipfs_cid_is_invalid() {
    ExtBuilder::build().execute_with(|| {
        assert_noop!(_create_profile(
                None,
                Some(invalid_content_ipfs())
            ), UtilsError::<TestRuntime>::InvalidIpfsCid);
    });
}

// Update profile tests
// ----------------------------------------------------------------------------

#[test]
fn update_profile_should_work() {
    ExtBuilder::build().execute_with(|| {
        assert_ok!(_create_default_profile());
        // AccountId 1
        assert_ok!(_update_profile(
                None,
                Some(space_content_ipfs())
            ));

        // Check whether profile updated correctly
        let profile = Profiles::social_account_by_id(ACCOUNT1).unwrap().profile.unwrap();
        assert!(profile.updated.is_some());
        assert_eq!(profile.content, space_content_ipfs());

        // Check whether profile history is written correctly
        let profile_history = ProfileHistory::edit_history(ACCOUNT1)[0].clone();
        assert_eq!(profile_history.old_data.content, Some(profile_content_ipfs()));
    });
}

#[test]
fn update_profile_should_fail_when_social_account_not_found() {
    ExtBuilder::build().execute_with(|| {
        assert_noop!(_update_profile(
                None,
                Some(profile_content_ipfs())
            ), ProfilesError::<TestRuntime>::SocialAccountNotFound);
    });
}

#[test]
fn update_profile_should_fail_when_account_has_no_profile() {
    ExtBuilder::build().execute_with(|| {
        assert_ok!(ProfileFollows::follow_account(Origin::signed(ACCOUNT1), ACCOUNT2));
        assert_noop!(_update_profile(
                None,
                Some(profile_content_ipfs())
            ), ProfilesError::<TestRuntime>::AccountHasNoProfile);
    });
}

#[test]
fn update_profile_should_fail_when_no_updates_for_profile_provided() {
    ExtBuilder::build().execute_with(|| {
        assert_ok!(_create_default_profile());
        // AccountId 1
        assert_noop!(_update_profile(
                None,
                None
            ), ProfilesError::<TestRuntime>::NoUpdatesForProfile);
    });
}

#[test]
fn update_profile_should_fail_when_ipfs_cid_is_invalid() {
    ExtBuilder::build().execute_with(|| {
        assert_ok!(_create_default_profile());
        assert_noop!(_update_profile(
                None,
                Some(invalid_content_ipfs())
            ), UtilsError::<TestRuntime>::InvalidIpfsCid);
    });
}

// Create social account tests
// ----------------------------------------------------------------------------

#[test]
fn create_social_account_should_work_with_without_referrer_and_drip_amount() {
    ExtBuilder::build_with_one_default_social_account().execute_with(|| {
        check_profile_storages(ACCOUNT4, None, 0, NEXT_PERIOD);
    });
}

#[test]
fn create_social_account_should_work_with_referrer() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_ok!(_create_social_account(
            None,
            None,
            Some(Some(ACCOUNT2)),
            None
        ));

        check_profile_storages(ACCOUNT4, Some(ACCOUNT2), 0, NEXT_PERIOD);
    });
}

#[test]
fn create_social_account_should_work_with_drip_amount() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_eq!(Balances::free_balance(ACCOUNT4), 0);

        assert_ok!(_create_social_account(
            None,
            None,
            None,
            Some(Some(default_faucet().drip_limit))
        ));

        let drip_limit = default_faucet().drip_limit;
        check_profile_storages(ACCOUNT4, None, drip_limit, NEXT_PERIOD);
    });
}

#[test]
fn create_social_account_should_work_with_referrer_and_drip_amount() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_eq!(Balances::free_balance(ACCOUNT4), 0);

        assert_ok!(_create_social_account(
            None,
            None,
            Some(Some(ACCOUNT2)),
            Some(Some(default_faucet().drip_limit))
        ));

        let drip_limit = default_faucet().drip_limit;
        check_profile_storages(ACCOUNT4, Some(ACCOUNT2), drip_limit, NEXT_PERIOD);
    });
}

#[test]
fn create_social_account_should_work_when_next_period_will_come() {
    ExtBuilder::build_with_one_default_social_account().execute_with(|| {
        System::set_block_number(NEXT_PERIOD);

        assert_ok!(_create_social_account(
            None,
            Some(ACCOUNT5),
            Some(Some(ACCOUNT2)),
            None
        ));

        check_profile_storages(ACCOUNT5, Some(ACCOUNT2), 0, NEXT_PERIOD + PERIOD);
    });
}

#[test]
fn create_social_account_should_fail_with_period_limit_reached() {
    ExtBuilder::build_with_one_default_social_account().execute_with(|| {
        assert_noop!(_create_social_account(
            None,
            Some(ACCOUNT3),
            Some(Some(ACCOUNT2)),
            None
        ), ProfilesError::<TestRuntime>::PeriodLimitReached);
    });
}

#[test]
fn create_social_account_should_fail_with_not_member() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_noop!(_create_social_account(
            Some(Origin::signed(FAUCET2)),
            None,
            None,
            None
        ), ProfilesError::<TestRuntime>::NotMember);
    });
}
