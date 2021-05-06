use pallet_utils::{SpaceId, Content};

pub trait IsAccountBlocked<AccountId> {
    fn is_blocked_account(account: AccountId, scope: SpaceId) -> bool;
    fn is_allowed_account(account: AccountId, scope: SpaceId) -> bool;
}

impl<AccountId> IsAccountBlocked<AccountId> for () {
    fn is_blocked_account(_account: AccountId, _scope: u64) -> bool {
        false
    }

    fn is_allowed_account(_account: AccountId, _scope: u64) -> bool {
        true
    }
}

pub trait IsSpaceBlocked {
    fn is_blocked_space(space_id: SpaceId, scope: SpaceId) -> bool;
    fn is_allowed_space(space_id: SpaceId, scope: SpaceId) -> bool;
}

// TODO: reuse `type PostId` from pallet_utils in future updates
pub trait IsPostBlocked<PostId> {
    fn is_blocked_post(post_id: PostId, scope: SpaceId) -> bool;
    fn is_allowed_post(post_id: PostId, scope: SpaceId) -> bool;
}

impl<PostId> IsPostBlocked<PostId> for () {
    fn is_blocked_post(_post_id: PostId, _scope: SpaceId) -> bool {
        false
    }

    fn is_allowed_post(_post_id: PostId, _scope: u64) -> bool {
        true
    }
}

pub trait IsContentBlocked {
    fn is_blocked_content(content: Content, scope: SpaceId) -> bool;
    fn is_allowed_content(content: Content, scope: SpaceId) -> bool;
}

impl IsContentBlocked for () {
    fn is_blocked_content(_content: Content, _scope: u64) -> bool {
        false
    }
    fn is_allowed_content(_content: Content, _scope: SpaceId) -> bool {
        true
    }
}
