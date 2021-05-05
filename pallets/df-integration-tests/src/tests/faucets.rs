use super::*;

use sp_runtime::{
    DispatchError::BadOrigin
};

use pallet_faucets::{FaucetUpdate, Faucet, Error as FaucetsError};

impl pallet_faucets::Trait for TestRuntime {
    type Event = ();
}

impl ExtBuilder {
    pub fn build_with_faucet() -> TestExternalities {
        let mut ext = Self::build();
        ext.execute_with(|| {
            assert_ok!(_add_default_faucet());
            assert_ok!(OffchainMembership::add_member(Origin::signed(SUDO_ACCOUNT), FAUCET1));
        });
        ext
    }

    // TODO do we really need this func? it's much clearer to call _do_default_drip() in a few tests directly.
    pub fn build_with_one_default_drip() -> TestExternalities {
        let mut ext = Self::build_with_faucet();

        ext.execute_with(|| {
            System::set_block_number(BLOCK_NUMBER_BEFORE_DRIP);
            assert_ok!(_do_default_drip());
        });

        ext
    }
}

pub(crate) const FAUCET_INITIAL_BALANCE: Balance = 400;

pub(crate) const FAUCET1: AccountId = 101;
pub(crate) const FAUCET2: AccountId = 102;
pub(crate) const FAUCET8: AccountId = 108;
pub(crate) const FAUCET9: AccountId = 109;

pub(crate) const ACCOUNT1: AccountId = 111;

pub(crate) const BLOCK_NUMBER_BEFORE_DRIP: BlockNumber = 20;

pub(crate) const fn default_faucet() -> Faucet<TestRuntime> {
    Faucet {
        enabled: true,
        period: 100,
        period_limit: 50,
        drip_limit: 25,

        next_period_at: 0,
        dripped_in_current_period: 0,
    }
}

pub(crate) const fn default_faucet_update() -> FaucetUpdate<BlockNumber, Balance> {
    FaucetUpdate {
        enabled: None,
        period: Some(7_200),
        period_limit: Some(100),
        drip_limit: Some(50)
    }
}

pub(crate) fn _add_default_faucet() -> DispatchResult {
    _add_faucet(None, None)
}

pub(crate) fn _add_faucet(
    origin: Option<Origin>,
    faucet_account: Option<AccountId>,
) -> DispatchResult {
    let settings =  default_faucet();
    Faucets::add_faucet(
        origin.unwrap_or_else(Origin::root),
        faucet_account.unwrap_or(FAUCET1),
        settings.period,
        settings.period_limit,
        settings.drip_limit
    )
}

pub(crate) fn _update_default_faucet() -> DispatchResult {
    _update_faucet(None, None, None)
}

pub(crate) fn _update_faucet_settings(settings: FaucetUpdate<BlockNumber, Balance>) -> DispatchResult {
    _update_faucet(None, None, Some(settings))
}

pub(crate) fn _update_faucet(
    origin: Option<Origin>,
    faucet_account: Option<AccountId>,
    update: Option<FaucetUpdate<BlockNumber, Balance>>
) -> DispatchResult {
    Faucets::update_faucet(
        origin.unwrap_or_else(Origin::root),
        faucet_account.unwrap_or(FAUCET1),
        update.unwrap_or_else(default_faucet_update),
    )
}

pub(crate) fn _remove_default_faucet() -> DispatchResult {
    _remove_faucets(None, None)
}

pub(crate) fn _remove_faucets(
    origin: Option<Origin>,
    faucet_accounts: Option<Vec<AccountId>>,
) -> DispatchResult {
    Faucets::remove_faucets(
        origin.unwrap_or_else(Origin::root),
        faucet_accounts.unwrap_or_else(|| vec![FAUCET1])
    )
}

pub(crate) fn _do_default_drip() -> DispatchResult {
    _drip(None, None, None)
}

pub(crate) fn _drip(
    origin: Option<Origin>,
    recipient: Option<AccountId>,
    amount: Option<Balance>
) -> DispatchResult {
    Faucets::drip(
        origin.unwrap_or_else(|| Origin::signed(FAUCET1)),
        recipient.unwrap_or(ACCOUNT1),
        amount.unwrap_or(default_faucet().drip_limit)
    )
}

// Add faucet
// ----------------------------------------------------------------------------

#[test]
fn add_faucet_should_work() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        let faucet = Faucets::faucet_by_account(FAUCET1).unwrap();
        assert!(faucet == default_faucet());
    });
}

#[test]
fn add_faucet_should_fail_when_origin_is_not_root() {
    ExtBuilder::build().execute_with(|| {
        let not_root = Origin::signed(ACCOUNT1);
        assert_noop!(
            _add_faucet(Some(not_root), None),
            BadOrigin
        );
    });
}

#[test]
fn add_faucet_should_fail_when_faucet_already_added() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_noop!(
            _add_default_faucet(),
            FaucetsError::<TestRuntime>::FaucetAlreadyAdded
        );
    });
}

#[test]
fn add_faucet_should_fail_when_no_free_balance_on_account() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_noop!(
            _add_faucet(None, Some(FAUCET9)),
            FaucetsError::<TestRuntime>::NoFreeBalanceOnFaucet
        );
    });
}

// Update faucet
// ----------------------------------------------------------------------------

#[test]
fn update_faucet_should_work() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_ok!(_update_default_faucet());
        let update = default_faucet_update();

        let faucet = Faucets::faucet_by_account(FAUCET1).unwrap();
        let updated_faucet = Faucet::<TestRuntime>::new(
            update.period.unwrap_or(faucet.period),
            update.period_limit.unwrap_or(faucet.period_limit),
            update.drip_limit.unwrap_or(faucet.drip_limit)
        );

        assert_eq!(faucet.period, updated_faucet.period);
        assert_eq!(faucet.period_limit, updated_faucet.period_limit);
        assert_eq!(faucet.drip_limit, updated_faucet.drip_limit);
    });
}

#[test]
fn update_faucet_should_fail_when_no_updates_provided() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_noop!(
            _update_faucet_settings(
                FaucetUpdate {
                    enabled: None,
                    period: None,
                    period_limit: None,
                    drip_limit: None
                }
            ),
            FaucetsError::<TestRuntime>::NoUpdatesProvided
        );
    });
}

#[test]
fn update_faucet_should_fail_when_faucet_address_in_unknown() {
    ExtBuilder::build().execute_with(|| {
        assert_noop!(
            _update_default_faucet(),
            FaucetsError::<TestRuntime>::FaucetNotFound
        );
    });
}

#[test]
fn update_faucet_should_fail_when_same_active_flag_provided() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_noop!(
            _update_faucet_settings(
                FaucetUpdate {
                    enabled: Some(default_faucet().enabled),
                    period: None,
                    period_limit: None,
                    drip_limit: None
                }
            ),
            FaucetsError::<TestRuntime>::NothingToUpdate
        );
    });
}

#[test]
fn update_faucet_should_fail_when_same_period_provided() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_noop!(
            _update_faucet_settings(
                FaucetUpdate {
                    enabled: None,
                    period: Some(default_faucet().period),
                    period_limit: None,
                    drip_limit: None
                }
            ),
            FaucetsError::<TestRuntime>::NothingToUpdate
        );
    });
}

#[test]
fn update_faucet_should_fail_when_same_period_limit_provided() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_noop!(
            _update_faucet_settings(
                FaucetUpdate {
                    enabled: None,
                    period: None,
                    period_limit: Some(default_faucet().period_limit),
                    drip_limit: None
                }
            ),
            FaucetsError::<TestRuntime>::NothingToUpdate
        );
    });
}

#[test]
fn update_faucet_should_fail_when_same_drip_limit_provided() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_noop!(
            _update_faucet_settings(
                FaucetUpdate {
                    enabled: None,
                    period: None,
                    period_limit: None,
                    drip_limit: Some(default_faucet().drip_limit)
                }
            ),
            FaucetsError::<TestRuntime>::NothingToUpdate
        );
    });
}

// Remove faucets
// ----------------------------------------------------------------------------

#[test]
fn remove_faucets_should_work() {
    ExtBuilder::build().execute_with(|| {
        // This will add faucets with accounts ids [1; 8]
        let mut faucets = Vec::new();
        for account in FAUCET1..=FAUCET8 {
            assert_ok!(_add_faucet(None, Some(account)));
            faucets.push(account);
        }

        // This should remove only faucets from 1 to 7
        let _ = faucets.pop();
        assert_ok!(
            _remove_faucets(
                None,
                Some(faucets)
            )
        );

        for account in FAUCET1..FAUCET8 {
            assert!(Faucets::faucet_by_account(account).is_none());
        }
        assert!(Faucets::faucet_by_account(FAUCET8).is_some());
    });
}

#[test]
fn remove_faucets_should_handle_duplicate_addresses() {
    ExtBuilder::build().execute_with(|| {
        // This will add faucets with accounts ids [1; 8]
        let mut faucets = Vec::new();
        for account in FAUCET1..=FAUCET8 {
            assert_ok!(_add_faucet(None, Some(account)));
            faucets.push(account);
        }

        // This should remove only faucets from 1 to 7
        let _ = faucets.pop();
        let mut duplicates = vec![FAUCET1, FAUCET2];
        faucets.append(&mut duplicates);
        assert_ok!(
            _remove_faucets(
                None,
                Some(faucets)
            )
        );

        for account in FAUCET1..FAUCET8 {
            assert!(Faucets::faucet_by_account(account).is_none());
        }
        assert!(Faucets::faucet_by_account(FAUCET8).is_some());
    });
}

#[test]
fn remove_faucets_should_fail_when_no_faucet_addresses_provided() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_noop!(
            _remove_faucets(
                None,
                Some(vec![])
            ),
            FaucetsError::<TestRuntime>::NoFaucetsProvided
        );
    });
}

// Drip
// ----------------------------------------------------------------------------

#[test]
fn drip_should_work() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        System::set_block_number(BLOCK_NUMBER_BEFORE_DRIP);

        let Faucet { period, drip_limit, .. } = default_faucet();
        assert_eq!(Balances::free_balance(ACCOUNT1), 0);

        assert_ok!(_do_default_drip());

        assert_eq!(Balances::free_balance(ACCOUNT1), drip_limit);

        let faucet_state = Faucets::faucet_by_account(FAUCET1).unwrap();
        assert_eq!(faucet_state.next_period_at, BLOCK_NUMBER_BEFORE_DRIP + period);
        assert_eq!(faucet_state.dripped_in_current_period, drip_limit);
    });
}

#[test]
fn drip_should_work_multiple_times_in_same_period() {
    ExtBuilder::build_with_one_default_drip().execute_with(|| {
        let Faucet { period, drip_limit, .. } = default_faucet();

        // Do the second drip
        assert_ok!(_drip(None, None, Some(drip_limit)));
        assert_eq!(Balances::free_balance(ACCOUNT1), drip_limit * 2);

        let faucet_state = Faucets::faucet_by_account(FAUCET1).unwrap();
        assert_eq!(faucet_state.next_period_at, BLOCK_NUMBER_BEFORE_DRIP + period);
        assert_eq!(faucet_state.dripped_in_current_period, drip_limit * 2);
    });
}

#[test]
fn drip_should_work_for_same_recipient_in_next_period() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        System::set_block_number(BLOCK_NUMBER_BEFORE_DRIP);

        let Faucet { period, drip_limit, .. } = default_faucet();

        // Drip to the same recipient twice in the same period to reach period limit
        assert_ok!(_do_default_drip());
        assert_ok!(_do_default_drip());
        assert_eq!(Balances::free_balance(ACCOUNT1), drip_limit * 2);

        // Move to the next period
        System::set_block_number(BLOCK_NUMBER_BEFORE_DRIP + period);

        // Repeat the same drip as we did a few line above but now it will be in the next period
        assert_ok!(_do_default_drip());
        assert_eq!(Balances::free_balance(ACCOUNT1), drip_limit * 3);

        let faucet_state = Faucets::faucet_by_account(FAUCET1).unwrap();
        assert_eq!(faucet_state.next_period_at, BLOCK_NUMBER_BEFORE_DRIP + period * 2);
        assert_eq!(faucet_state.dripped_in_current_period, drip_limit);
    });
}

#[test]
fn drip_should_fail_when_period_limit_reached() {
    ExtBuilder::build_with_one_default_drip().execute_with(|| {
        System::set_block_number(BLOCK_NUMBER_BEFORE_DRIP);

        // Do the second drip
        assert_ok!(_do_default_drip());

        // The third drip should fail, b/c it exceeds the period limit of this faucet
        assert_noop!(
            _do_default_drip(),
            FaucetsError::<TestRuntime>::PeriodLimitReached
        );

        let drip_limit = default_faucet().drip_limit;

        // Balance should be unchanged and equal to two drip
        assert_eq!(Balances::free_balance(ACCOUNT1), drip_limit * 2);
    });
}

#[test]
fn drip_should_fail_when_recipient_equals_faucet() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_noop!(
            _drip(None, Some(FAUCET1), None),
            FaucetsError::<TestRuntime>::RecipientEqualsFaucet
        );

        // Account should have no tokens if drip failed
        assert_eq!(Balances::free_balance(ACCOUNT1), 0);
    });
}

#[test]
fn drip_should_fail_when_amount_is_bigger_than_free_balance_on_faucet() {
    ExtBuilder::build_with_faucet().execute_with(|| {

        // Let's transfer most of tokens from the default faucet to another one
        assert_ok!(Balances::transfer(
            Origin::signed(FAUCET1),
            FAUCET2,
            FAUCET_INITIAL_BALANCE - 1 // Leave one token on the Faucet number 1.
        ));

        assert_noop!(
            _do_default_drip(),
            FaucetsError::<TestRuntime>::NotEnoughFreeBalanceOnFaucet
        );

        // Account should have no tokens if drip failed
        assert_eq!(Balances::free_balance(ACCOUNT1), 0);
    });
}

#[test]
fn drip_should_fail_when_zero_amount_provided() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        assert_noop!(
            _drip(None, None, Some(0)),
            FaucetsError::<TestRuntime>::ZeroDripAmountProvided
        );

        // Account should have no tokens if drip failed
        assert_eq!(Balances::free_balance(ACCOUNT1), 0);
    });
}

#[test]
fn drip_should_fail_when_too_big_amount_provided() {
    ExtBuilder::build_with_faucet().execute_with(|| {
        let too_big_amount = default_faucet().drip_limit + 1;
        assert_noop!(
            _drip(None, None, Some(too_big_amount)),
            FaucetsError::<TestRuntime>::DripLimitReached
        );

        // Account should have no tokens if drip failed
        assert_eq!(Balances::free_balance(ACCOUNT1), 0);
    });
}

#[test]
fn drip_should_fail_when_faucet_is_disabled_and_work_again_after_faucet_enabled() {
    ExtBuilder::build_with_faucet().execute_with(|| {

        // Account should have no tokens by default
        assert_eq!(Balances::free_balance(ACCOUNT1), 0);

        // Disable the faucet, so it will be not possible to drip
        assert_ok!(_update_faucet_settings(
            FaucetUpdate {
                enabled: Some(false),
                period: None,
                period_limit: None,
                drip_limit: None
            }
        ));

        // Faucet should not drip tokens if it is disabled
        assert_noop!(
            _do_default_drip(),
            FaucetsError::<TestRuntime>::FaucetDisabled
        );

        // Account should not receive any tokens
        assert_eq!(Balances::free_balance(ACCOUNT1), 0);

        // Make the faucet enabled again
        assert_ok!(_update_faucet_settings(
            FaucetUpdate {
                enabled: Some(true),
                period: None,
                period_limit: None,
                drip_limit: None
            }
        ));

        // Should be able to drip again
        assert_ok!(_do_default_drip());

        // Account should receive the tokens
        assert_eq!(Balances::free_balance(ACCOUNT1), default_faucet().drip_limit);
    });
}
