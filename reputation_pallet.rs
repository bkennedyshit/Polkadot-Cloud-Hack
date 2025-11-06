#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct ReputationProfile<AccountId> {
        pub total_score: u32,
        pub review_count: u32,
        pub category_scores: CategoryScores,
        pub active: bool,
        pub staked_amount: u128,
        pub owner: AccountId,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct CategoryScores {
        pub communication: u32,
        pub reliability: u32,
        pub quality: u32,
        pub professionalism: u32,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Rating<AccountId> {
        pub from: AccountId,
        pub to: AccountId,
        pub score: u8,
        pub category_ratings: CategoryScores,
        pub timestamp: u64,
        pub review_hash: [u8; 32], // IPFS hash of detailed review
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        #[pallet::constant]
        type MinStakeAmount: Get<u128>;
        
        #[pallet::constant]
        type MaxReviewsPerUser: Get<u32>;
    }

    #[pallet::storage]
    #[pallet::getter(fn user_reputation)]
    pub type UserReputation<T: Config> = 
        StorageMap<_, Blake2_128Concat, T::AccountId, ReputationProfile<T::AccountId>>;

    #[pallet::storage]
    #[pallet::getter(fn ratings)]
    pub type Ratings<T: Config> = 
        StorageDoubleMap<
            _,
            Blake2_128Concat, T::AccountId,  // rated user
            Blake2_128Concat, T::AccountId,  // rater
            Rating<T::AccountId>,
        >;

    #[pallet::storage]
    #[pallet::getter(fn rating_count)]
    pub type RatingCount<T: Config> = 
        StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProfileCreated { who: T::AccountId },
        RatingSubmitted { from: T::AccountId, to: T::AccountId, score: u8 },
        ReputationStaked { who: T::AccountId, amount: u128 },
        ReputationUnstaked { who: T::AccountId, amount: u128 },
        ProfileDeactivated { who: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        ProfileAlreadyExists,
        ProfileNotFound,
        CannotRateSelf,
        AlreadyRated,
        InvalidScore,
        InsufficientStake,
        ProfileNotActive,
        TooManyReviews,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn create_profile(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!UserReputation::<T>::contains_key(&who), Error::<T>::ProfileAlreadyExists);

            let profile = ReputationProfile {
                total_score: 0,
                review_count: 0,
                category_scores: CategoryScores {
                    communication: 0,
                    reliability: 0,
                    quality: 0,
                    professionalism: 0,
                },
                active: true,
                staked_amount: 0,
                owner: who.clone(),
            };

            UserReputation::<T>::insert(&who, profile);
            Self::deposit_event(Event::ProfileCreated { who });

            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn submit_rating(
            origin: OriginFor<T>,
            target: T::AccountId,
            score: u8,
            communication: u8,
            reliability: u8,
            quality: u8,
            professionalism: u8,
            review_hash: [u8; 32],
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(who != target, Error::<T>::CannotRateSelf);
            ensure!(score >= 1 && score <= 5, Error::<T>::InvalidScore);
            ensure!(UserReputation::<T>::contains_key(&target), Error::<T>::ProfileNotFound);
            ensure!(!Ratings::<T>::contains_key(&target, &who), Error::<T>::AlreadyRated);

            let mut profile = UserReputation::<T>::get(&target)
                .ok_or(Error::<T>::ProfileNotFound)?;

            ensure!(profile.active, Error::<T>::ProfileNotActive);

            let current_count = RatingCount::<T>::get(&target);
            ensure!(current_count < T::MaxReviewsPerUser::get(), Error::<T>::TooManyReviews);

            // Update reputation
            profile.total_score += score as u32;
            profile.review_count += 1;
            profile.category_scores.communication += communication as u32;
            profile.category_scores.reliability += reliability as u32;
            profile.category_scores.quality += quality as u32;
            profile.category_scores.professionalism += professionalism as u32;

            let rating = Rating {
                from: who.clone(),
                to: target.clone(),
                score,
                category_ratings: CategoryScores {
                    communication: communication as u32,
                    reliability: reliability as u32,
                    quality: quality as u32,
                    professionalism: professionalism as u32,
                },
                timestamp: Self::get_timestamp(),
                review_hash,
            };

            Ratings::<T>::insert(&target, &who, rating);
            UserReputation::<T>::insert(&target, profile);
            RatingCount::<T>::insert(&target, current_count + 1);

            Self::deposit_event(Event::RatingSubmitted { from: who, to: target, score });

            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn stake_reputation(origin: OriginFor<T>, amount: u128) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(amount >= T::MinStakeAmount::get(), Error::<T>::InsufficientStake);

            let mut profile = UserReputation::<T>::get(&who)
                .ok_or(Error::<T>::ProfileNotFound)?;

            profile.staked_amount += amount;
            UserReputation::<T>::insert(&who, profile);

            Self::deposit_event(Event::ReputationStaked { who, amount });

            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(3)]
        pub fn deactivate_profile(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let mut profile = UserReputation::<T>::get(&who)
                .ok_or(Error::<T>::ProfileNotFound)?;

            profile.active = false;
            UserReputation::<T>::insert(&who, profile);

            Self::deposit_event(Event::ProfileDeactivated { who });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn get_timestamp() -> u64 {
            // In production, use pallet_timestamp
            0
        }

        pub fn get_average_score(who: &T::AccountId) -> Option<u32> {
            UserReputation::<T>::get(who).map(|profile| {
                if profile.review_count == 0 {
                    0
                } else {
                    profile.total_score / profile.review_count
                }
            })
        }

        pub fn get_category_average(who: &T::AccountId) -> Option<CategoryScores> {
            UserReputation::<T>::get(who).map(|profile| {
                let count = profile.review_count.max(1);
                CategoryScores {
                    communication: profile.category_scores.communication / count,
                    reliability: profile.category_scores.reliability / count,
                    quality: profile.category_scores.quality / count,
                    professionalism: profile.category_scores.professionalism / count,
                }
            })
        }
    }
}