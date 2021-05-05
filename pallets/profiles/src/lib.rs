#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure,
    dispatch::DispatchResult,
    traits::{Currency, Get},
};
use sp_runtime::{
    traits::{Saturating, Zero},
    RuntimeDebug,
};
use sp_std::prelude::*;
use frame_system::{self as system, Module as System, ensure_signed};

use df_traits::FaucetsProvider;
use pallet_utils::{Module as Utils, WhoAndWhen, Content};

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct SocialAccount<T: Trait> {
    pub followers_count: u32,
    pub following_accounts_count: u16,
    pub following_spaces_count: u16,
    pub reputation: u32,
    pub profile: Option<Profile<T>>,
    pub created_at: T::BlockNumber,
    pub referrer: Option<T::AccountId>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct Profile<T: Trait> {
    pub created: WhoAndWhen<T>,
    pub updated: Option<WhoAndWhen<T>>,
    pub content: Content
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct ProfileUpdate {
    pub content: Option<Content>,
}

type BalanceOf<T> = <<T as pallet_utils::Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

/// The pallet's configuration trait.
pub trait Trait: system::Trait
    + pallet_utils::Trait
{
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type AfterProfileUpdated: AfterProfileUpdated<Self>;

    type MaxCreationsPerPeriod: Get<u32>;

    type BlocksInPeriod: Get<Self::BlockNumber>;

    type FaucetsProvider: FaucetsProvider<Self::AccountId, BalanceOf<Self>>;

    type AddSocialAccountMembers: Get<Vec<Self::AccountId>>;
}

// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as ProfilesModule {
        pub NextPeriodAt get(fn next_period_at): T::BlockNumber = Zero::zero();

        pub CreatedInCurrentPeriod get(fn created_in_current_period): u32 = 0;

        pub SocialAccountById get(fn social_account_by_id):
            map hasher(blake2_128_concat) T::AccountId => Option<SocialAccount<T>>;
    }
}

decl_event!(
    pub enum Event<T> where
        <T as system::Trait>::AccountId,
    {
        ProfileCreated(AccountId),
        ProfileUpdated(AccountId),
        SocialAccountCreated(/* New Account */ AccountId, /* Referrer */ Option<AccountId>),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Social account was not found by id.
        SocialAccountNotFound,
        /// Profile is already created for this account.
        ProfileAlreadyCreated,
        /// Nothing to update in a profile.
        NoUpdatesForProfile,
        /// Account has no profile yet.
        AccountHasNoProfile,
        /// Too many accounts created in this limit.
        PeriodLimitReached,
        /// Account is not permitted to add new social accounts.
        NotMember,
    }
}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    const MaxCreationsPerPeriod: u32 = T::MaxCreationsPerPeriod::get();

    const BlocksInPeriod: T::BlockNumber = T::BlocksInPeriod::get();

    const AddSocialAccountMembers: Vec<T::AccountId> = T::AddSocialAccountMembers::get();

    // Initializing errors
    type Error = Error<T>;

    // Initializing events
    fn deposit_event() = default;

    #[weight = 100_000 + T::DbWeight::get().reads_writes(1, 2)]
    pub fn create_profile(origin, content: Content) -> DispatchResult {
      let owner = ensure_signed(origin)?;

      Utils::<T>::is_valid_content(content.clone())?;

      let mut social_account = Self::get_or_new_social_account(owner.clone(),  None, None);
      ensure!(social_account.profile.is_none(), Error::<T>::ProfileAlreadyCreated);

      social_account.profile = Some(
        Profile {
          created: WhoAndWhen::<T>::new(owner.clone()),
          updated: None,
          content
        }
      );
      <SocialAccountById<T>>::insert(owner.clone(), social_account);

      Self::deposit_event(RawEvent::ProfileCreated(owner));
      Ok(())
    }

    #[weight = 100_000 + T::DbWeight::get().reads_writes(1, 2)]
    pub fn update_profile(origin, update: ProfileUpdate) -> DispatchResult {
      let owner = ensure_signed(origin)?;

      let has_updates = update.content.is_some();

      ensure!(has_updates, Error::<T>::NoUpdatesForProfile);

      let mut social_account = Self::social_account_by_id(owner.clone()).ok_or(Error::<T>::SocialAccountNotFound)?;
      let mut profile = social_account.profile.ok_or(Error::<T>::AccountHasNoProfile)?;
      let mut is_update_applied = false;
      let mut old_data = ProfileUpdate::default();

      if let Some(content) = update.content {
        if content != profile.content {
          Utils::<T>::is_valid_content(content.clone())?;
          old_data.content = Some(profile.content);
          profile.content = content;
          is_update_applied = true;
        }
      }

      if is_update_applied {
        profile.updated = Some(WhoAndWhen::<T>::new(owner.clone()));
        social_account.profile = Some(profile.clone());

        <SocialAccountById<T>>::insert(owner.clone(), social_account);
        T::AfterProfileUpdated::after_profile_updated(owner.clone(), &profile, old_data);

        Self::deposit_event(RawEvent::ProfileUpdated(owner));
      }
      Ok(())
    }

    #[weight = 50_000 + T::DbWeight::get().reads_writes(7, 5)]
    pub fn create_social_account(
      origin,
      new_account: T::AccountId,
      referrer: Option<T::AccountId>,
      drip_amount: Option<BalanceOf<T>>
    ) -> DispatchResult {
      let who = ensure_signed(origin)?;
      let members = T::AddSocialAccountMembers::get();
      ensure!(members.contains(&who), Error::<T>::NotMember);

      let current_block = System::<T>::block_number();

      let mut next_period_at = Self::next_period_at();
      let mut created_in_current_period = Self::created_in_current_period();

      if next_period_at <= current_block {
        // Move to the next period and reset the period stats
        next_period_at = current_block.saturating_add(T::BlocksInPeriod::get());
        created_in_current_period = Zero::zero();
      }

      ensure!(created_in_current_period < T::MaxCreationsPerPeriod::get(), Error::<T>::PeriodLimitReached);

      let social_account = Self::get_or_new_social_account(new_account.clone(), None, referrer.clone());

      if let Some(amount) = drip_amount {
        T::FaucetsProvider::do_drip(who, new_account.clone(), amount)?;
      }

      if current_block == social_account.created_at {
        created_in_current_period = created_in_current_period.saturating_add(1);

        <SocialAccountById<T>>::insert(new_account.clone(), social_account);
        Self::deposit_event(RawEvent::SocialAccountCreated(new_account, referrer));
      }

      CreatedInCurrentPeriod::mutate(|n| *n = created_in_current_period);
      NextPeriodAt::<T>::mutate(|p| *p = next_period_at);

      Ok(())
    }
  }
}

impl <T: Trait> SocialAccount<T> {
    pub fn inc_followers(&mut self) {
        self.followers_count = self.followers_count.saturating_add(1);
    }

    pub fn dec_followers(&mut self) {
        self.followers_count = self.followers_count.saturating_sub(1);
    }

    pub fn inc_following_accounts(&mut self) {
        self.following_accounts_count = self.following_accounts_count.saturating_add(1);
    }

    pub fn dec_following_accounts(&mut self) {
        self.following_accounts_count = self.following_accounts_count.saturating_sub(1);
    }

    pub fn inc_following_spaces(&mut self) {
        self.following_spaces_count = self.following_spaces_count.saturating_add(1);
    }

    pub fn dec_following_spaces(&mut self) {
        self.following_spaces_count = self.following_spaces_count.saturating_sub(1);
    }
}

impl<T: Trait> SocialAccount<T> {
    #[allow(clippy::comparison_chain)]
    pub fn change_reputation(&mut self, diff: i16) {
        if diff > 0 {
            self.reputation = self.reputation.saturating_add(diff.abs() as u32);
        } else if diff < 0 {
            self.reputation = self.reputation.saturating_sub(diff.abs() as u32);
        }
    }
}

impl Default for ProfileUpdate {
    fn default() -> Self {
        ProfileUpdate {
            content: None
        }
    }
}

impl<T: Trait> Module<T> {
    /// If `created_at_opt` is `None`, then current block number is set.
    pub fn get_or_new_social_account(
        account: T::AccountId,
        created_at_opt: Option<T::BlockNumber>,
        referrer: Option<T::AccountId>,
    ) -> SocialAccount<T> {
        Self::social_account_by_id(account).unwrap_or(
            SocialAccount {
                created_at: created_at_opt.unwrap_or_else(|| System::<T>::block_number()),
                followers_count: 0,
                following_accounts_count: 0,
                following_spaces_count: 0,
                reputation: 1,
                profile: None,
                referrer,
            }
        )
    }
}

#[impl_trait_for_tuples::impl_for_tuples(10)]
pub trait AfterProfileUpdated<T: Trait> {
    fn after_profile_updated(account: T::AccountId, post: &Profile<T>, old_data: ProfileUpdate);
}
