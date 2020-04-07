#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{ensure, decl_module, decl_error, decl_storage, decl_event};
use system::ensure_signed;
use sp_std::vec::Vec;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event! {
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        ClaimCreated(AccountId, Vec<u8>),
        ClaimRevoked(AccountId, Vec<u8>),
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        ProofAlreadyClaimed,
        NoSuchProof,
        NotProofOwner,
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        Proofs: map hasher(opaque_blake2_256) Vec<u8> => (T::AccountId, T::BlockNumber);
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        fn create_claim(origin, proof: Vec<u8>) {
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            let current_block = <system::Module<T>>::block_number();

            Proofs::<T>::insert(&proof, (sender.clone(), current_block));

            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
        }

        fn revoke_claim(origin, proof: Vec<u8>) {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            let (owner, _) = Proofs::<T>::get(&proof);

            ensure!(sender == owner, Error::<T>::NotProofOwner);

            Proofs::<T>::remove(&proof);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
        }
    }
}