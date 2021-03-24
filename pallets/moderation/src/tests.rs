use crate::{Error, mock::*};
use crate::*;

use frame_support::{assert_ok, assert_noop};
use pallet_posts::PostById;
use pallet_utils::{Error as UtilsError};
use pallet_spaces::{SpaceById, Error as SpaceError};

#[test]
fn report_entity_should_work() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());

        assert_eq!(Moderation::next_report_id(), REPORT2);

        let report = Moderation::report_by_id(REPORT1).unwrap();
        assert_eq!(report.id, REPORT1);
        assert_eq!(report.created.account, ACCOUNT1);
        assert_eq!(report.reported_entity, EntityId::Post(POST1));
        assert_eq!(report.reported_within, SPACE1);
        assert_eq!(report.reason, valid_content_ipfs_1());
    });
}

#[test]
fn report_entity_should_fail_with_reason_is_empty() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_noop!(
            _report_entity(
                None,
                None,
                None,
                Some(Content::None)
            ), Error::<Test>::ReasonIsEmpty
        );
    });
}

#[test]
fn report_entity_should_fail_with_invalid_ipfs_cid() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_noop!(
            _report_entity(
                None,
                None,
                None,
                Some(invalid_content_ipfs())
            ), UtilsError::<Test>::InvalidIpfsCid
        );
    });
}

#[test]
fn report_entity_should_fail_with_invalid_scope() {
    ExtBuilder::build().execute_with(|| {
        assert_noop!(_report_default_entity(), Error::<Test>::InvalidScope);
    });
}

#[test]
fn report_entity_should_fail_with_already_reported_entity() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_noop!(_report_default_entity(), Error::<Test>::AlreadyReportedEntity);
    });
}

//-------------------------------------------------------------------------
#[test]
fn suggest_entity_status_should_work() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_ok!(_suggest_default_entity_status());

        let suggestions = Moderation::suggested_statuses(EntityId::Post(POST1), SPACE1);
        let suggested_status = SuggestedStatus::<Test>::new(
            ACCOUNT1,
            Some(EntityStatus::Blocked),
            Some(REPORT1),
        );

        assert!(suggestions == vec![suggested_status]);
    });
}

#[test]
fn suggest_entity_status_should_fail_with_report_not_found() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_noop!(
            _suggest_entity_status(
                None,
                None,
                None,
                None,
                Some(Some(REPORT2))
            ), Error::<Test>::ReportNotFound
        );
    });
}

#[test]
fn suggest_entity_status_should_fail_with_suggested_status_in_wrong_scope() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_noop!(
            _suggest_entity_status(
                None,
                None,
                Some(SPACE2),
                None,
                None
            ), Error::<Test>::SuggestedStatusInWrongScope
        );
    });
}

#[test]
fn suggest_entity_status_should_fail_with_suggested_same_entity_status() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_ok!(_suggest_default_entity_status());
        assert_ok!(_update_default_entity_status());
        assert_noop!(
            _suggest_entity_status(
                None,
                None,
                None,
                Some(Some(EntityStatus::Allowed)),
                None
            ), Error::<Test>::SuggestedSameEntityStatus
        );
    });
}

#[test]
fn suggest_entity_status_should_fail_with_invalid_scope() {
    ExtBuilder::build_with_report_no_space().execute_with(|| {
        assert_noop!(_suggest_default_entity_status(), Error::<Test>::InvalidScope);
    });
}

#[test]
fn suggest_entity_status_should_fail_with_no_permission_to_suggest_entity_status() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_noop!(
            _suggest_entity_status(
                Some(Origin::signed(ACCOUNT2)),
                None,
                None,
                None,
                None
            ), Error::<Test>::NoPermissionToSuggestEntityStatus
        );
    });
}

//----------------------------------------------------------------------------
#[test]
fn update_entity_status_should_work_status_allowed() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_ok!(_suggest_default_entity_status());
        assert_ok!(_update_default_entity_status());

        let status = Moderation::status_by_entity_in_space(EntityId::Post(POST1), SPACE1).unwrap();
        assert_eq!(status, EntityStatus::Allowed);
        // let post = PostById::<Test>::get(POST1).unwrap();
        // assert!(post.space_id.is_none());
    });
}

#[test]
fn update_entity_status_should_work_status_blocked() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_ok!(_suggest_default_entity_status());
        assert_ok!(
            _update_entity_status(
                None,
                None,
                None,
                Some(Some(EntityStatus::Blocked))
            )
        );

        let post = PostById::<Test>::get(POST1).unwrap();
        assert!(post.space_id.is_none());
    });
}

#[test]
fn update_entity_status_should_fail_with_invalid_scope() {
    ExtBuilder::build_with_report_no_space().execute_with(|| {
        assert_noop!(_update_default_entity_status(), Error::<Test>::InvalidScope);
    });
}

#[test]
fn update_entity_status_should_fail_with_no_permission_to_update_entity_status() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_noop!(
            _update_entity_status(
                Some(Origin::signed(ACCOUNT2)),
                None,
                None,
                None
            ), Error::<Test>::NoPermissionToUpdateEntityStatus
        );
    });
}

//---------------------------------------------------------------------------
#[test]
fn delete_entity_status_should_work() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_ok!(_suggest_default_entity_status());
        assert_ok!(_update_default_entity_status());
        assert_ok!(_delete_default_entity_status());

        let status = Moderation::status_by_entity_in_space(EntityId::Post(POST1), SPACE1);
        assert!(status.is_none());
    });
}

#[test]
fn delete_entity_status_should_fail_entity_has_no_status_in_scope() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_ok!(_suggest_default_entity_status());
        assert_noop!(_delete_default_entity_status(), Error::<Test>::EntityHasNoStatusInScope);
    });
}

#[test]
fn delete_entity_status_should_fail_invalid_scope() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_ok!(_suggest_default_entity_status());
        assert_ok!(_update_default_entity_status());
        SpaceById::<Test>::remove(SPACE1);
        assert_noop!(_delete_default_entity_status(), Error::<Test>::InvalidScope);
    });
}

#[test]
fn delete_entity_status_should_fail_no_permission_to_update_entity_status() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_report_default_entity());
        assert_ok!(_suggest_default_entity_status());
        assert_ok!(_update_default_entity_status());
        assert_noop!(
            _delete_entity_status(
                Some(Origin::signed(ACCOUNT2)),
                None,
                None
            ), Error::<Test>::NoPermissionToUpdateEntityStatus
        );
    });
}

//----------------------------------------------------------------------------
#[test]
fn update_moderation_settings_should_work() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_ok!(_update_default_moderation_settings());

        let settings = Moderation::moderation_settings(SPACE1).unwrap();
        assert_eq!(settings.autoblock_threshold, Some(AUTOBLOCK_THRESHOLD));
    });
}

#[test]
fn update_moderation_settings_should_fail_with_no_updates_for_moderation_settings() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_noop!(
            _update_moderation_settings(
                None,
                None,
                Some(empty_moderation_settings_update())
            ), Error::<Test>::NoUpdatesForModerationSettings
        );
    });
}

#[test]
fn update_moderation_settings_should_fail_with_space_not_found() {
    ExtBuilder::build_with_report_no_space().execute_with(|| {
        assert_noop!(_update_default_moderation_settings(), SpaceError::<Test>::SpaceNotFound);
    });
}

#[test]
fn update_moderation_settings_should_fail_with_no_permission_to_update_moderation_settings() {
    ExtBuilder::build_with_space_and_post().execute_with(|| {
        assert_noop!(
            _update_moderation_settings(
                Some(Origin::signed(ACCOUNT2)),
                None,
                None
            ), Error::<Test>::NoPermissionToUpdateModerationSettings
        );
    });
}
