use crate::{SpacePermission as SP, SpacePermissions, SpacePermissionSet};

use sp_std::iter::FromIterator;
use sp_std::vec;
use frame_support::parameter_types;

parameter_types! {
  pub DefaultSpacePermissions: SpacePermissions = SpacePermissions {

    // No permissions disabled by default
    none: None,

    everyone: Some(SpacePermissionSet::from_iter(vec![
      SP::UpdateOwnSubspaces,
      SP::DeleteOwnSubspaces,
      SP::HideOwnSubspaces,

      SP::UpdateOwnPosts,
      SP::DeleteOwnPosts,
      SP::HideOwnPosts,

      SP::CreateComments,
      SP::UpdateOwnComments,
      SP::DeleteOwnComments,
      SP::HideOwnComments,

      SP::Upvote,
      SP::Downvote,
      SP::Share,
    ].into_iter())),

    // Followers can do everything that everyone else can.
    follower: None,

    space_owner: Some(SpacePermissionSet::from_iter(vec![
      SP::ManageRoles,
      SP::RepresentSpaceInternally,
      SP::RepresentSpaceExternally,
      SP::OverrideSubspacePermissions,
      SP::OverridePostPermissions,

      SP::CreateSubspaces,
      SP::CreatePosts,

      SP::UpdateSpace,
      SP::UpdateAnySubspace,
      SP::UpdateAnyPost,

      SP::DeleteAnySubspace,
      SP::DeleteAnyPost,

      SP::HideAnySubspace,
      SP::HideAnyPost,
      SP::HideAnyComment,

      SP::SuggestEntityStatus,
      SP::UpdateEntityStatus,

      SP::UpdateSpaceSettings,
    ].into_iter())),
  };
}
