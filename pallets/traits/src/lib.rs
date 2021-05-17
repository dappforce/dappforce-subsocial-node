#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::{DispatchError, DispatchResult};

use pallet_permissions::{
  SpacePermission,
  SpacePermissionsContext
};
use pallet_utils::{SpaceId, User};

pub mod moderation;

pub trait SpaceFollowsProvider<AccountId> {
  fn is_space_follower(account: AccountId, space_id: SpaceId) -> bool;
}

impl<AccountId> SpaceFollowsProvider<AccountId> for () {
  fn is_space_follower(_account: AccountId, _space_id: u64) -> bool {
    true
  }
}

pub trait PermissionChecker {
  type AccountId;

  fn ensure_user_has_space_permission(
    user: User<Self::AccountId>,
    ctx: SpacePermissionsContext,
    permission: SpacePermission,
    error: DispatchError,
  ) -> DispatchResult;

  fn ensure_account_has_space_permission(
    account: Self::AccountId,
    ctx: SpacePermissionsContext,
    permission: SpacePermission,
    error: DispatchError,
  ) -> DispatchResult {

    Self::ensure_user_has_space_permission(
      User::Account(account),
      ctx,
      permission,
      error
    )
  }
}
