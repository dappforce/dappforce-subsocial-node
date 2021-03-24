use crate::{SpaceId};

use pallet_utils::Content;

pub trait IsAccountBlocked {
    type AccountId;

    fn is_account_blocked(account: Self::AccountId, scope: SpaceId) -> bool;
}

impl IsAccountBlocked for () {
    type AccountId = sp_runtime::AccountId32;

    fn is_account_blocked(_account: Self::AccountId, _scope: u64) -> bool {
        false
    }
}

pub trait IsSpaceBlocked {
    fn is_space_blocked(space_id: SpaceId, scope: SpaceId) -> bool;
}

pub trait IsPostBlocked {
    type PostId;

    fn is_post_blocked(post_id: Self::PostId, scope: SpaceId) -> bool;
}

impl IsPostBlocked for () {
    type PostId = u64;

    fn is_post_blocked(_post_id: Self::PostId, _scope: SpaceId) -> bool {
        false
    }
}

pub trait IsContentBlocked {
    fn is_content_blocked(content: Content, scope: SpaceId) -> bool;
}

impl IsContentBlocked for () {
    fn is_content_blocked(_content: Content, _scope: u64) -> bool {
        false
    }
}
