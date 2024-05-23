//! Brad's practice pallet

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
	use frame_support::{pallet_prelude::*, Parameter, dispatch::DispatchResultWithPostInfo};
	use frame_system::pallet_prelude::*;
    use sp_runtime::traits::AtLeast32BitUnsigned;
    use sp_runtime::traits::Saturating;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
	pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// This pallet's balance type
        type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;
		// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
    }

    /// Storage item for accounts to balances mapping
    #[pallet::storage]
    #[pallet::getter(fn get_balance)]
	pub(super) type Balances<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        T::Balance,
        ValueQuery,
    >;

    /// Token mint can emit two event types.
    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        /// New token supply was minted
		MintedNewSupply {
            account: T::AccountId,
        },
        /// Tokens were successfully transferred between accounts.
        Transferred {
            sender: T::AccountId,
            recipient: T::AccountId,
            transferred: T::Balance,
        }
	}

    #[pallet::error]
	pub enum Error<T> {
		DefaultError,
	}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Issue an amount of tokens from any origin.
		/// 
		/// This would not make sense to have in practice in the current
		/// implementation. This is an educational ressource.
		/// 
		/// Parameters:
		/// - `amount`: The amount of tokens to mint.
		///
		/// Emits `MintedNewSupply` event when successful.
		///
		/// TODO: Add safety checks and set max issuance allowed.  
		/// Weight: `O(1)`
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::mint())]
        pub fn mint(
            origin: OriginFor<T>,
            amount: T::Balance
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            // Update storage.
            <Balances<T>>::insert(&sender, amount);

            // Emit an event
            Self::deposit_event(Event::MintedNewSupply{ account: sender });

            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }

        /// Allow minting account to transfer a given balance to another account.
		///
		/// Parameters:
		/// - `to`: The account to receive the transfer.
		/// - `amount`: The amount of balance to transfer.
		///
		/// Emits `Transferred` event when successful.
		///
		/// TODO: Add checks on minimum balance required and maximum transferrable balance.  
		/// Weight: `O(1)`	
        #[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			#[pallet::compact] amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let sender_balance = Self::get_balance(&sender);
			let receiver_balance = Self::get_balance(&to);

			// Calculate new balances.
			let update_sender = sender_balance.saturating_sub(amount);
			let update_to = receiver_balance.saturating_add(amount);

			// Update both accounts storage.
			<Balances<T>>::insert(&sender, update_sender);
			<Balances<T>>::insert(&to, update_to);

			// Emit event.
			Self::deposit_event(Event::Transferred{ sender, recipient: to, transferred: amount });
			Ok(().into())
		}
    }
}