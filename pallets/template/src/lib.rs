//! 一个简易货币 pallet：任何人可铸币、持币者可转账。
//! 用于学习 FRAME 的 storage / event / error / call 四件套。

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;
use polkadot_sdk::polkadot_sdk_frame as frame;

pub use pallet::*;

#[frame::pallet]
pub mod pallet {
    use super::*;

    /// 余额类型
    pub type Balance = u128;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// 总发行量
    #[pallet::storage]
    pub type TotalIssuance<T> = StorageValue<Value = Balance>;

    /// 账号 -> 余额
    #[pallet::storage]
    pub type Balances<T: Config> =
        StorageMap<Hasher = Blake2_128Concat, Key = T::AccountId, Value = Balance>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// 铸币成功
        Minted { to: T::AccountId, amount: Balance },
        /// 转账成功
        Transferred { from: T::AccountId, to: T::AccountId, amount: Balance },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// 账号无余额记录
        NonExistentAccount,
        /// 余额不足
        InsufficientBalance,
        /// 算术溢出
        Overflow,
        /// 不能转给自己
        TransferToSelf,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 铸币。任何人可调用 —— 仅教学用，真实链必须加权限
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn mint_unsafe(
            origin: T::RuntimeOrigin,
            dest: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            let _anyone = ensure_signed(origin)?;

            let old = Balances::<T>::get(&dest).unwrap_or(0);
            Balances::<T>::insert(&dest, old.checked_add(amount).ok_or(Error::<T>::Overflow)?);

            let issuance = TotalIssuance::<T>::get().unwrap_or(0);
            TotalIssuance::<T>::put(issuance.checked_add(amount).ok_or(Error::<T>::Overflow)?);

            Self::deposit_event(Event::Minted { to: dest, amount });
            Ok(())
        }

        /// 转账
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn transfer(
            origin: T::RuntimeOrigin,
            dest: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(sender != dest, Error::<T>::TransferToSelf);

            let sender_balance =
                Balances::<T>::get(&sender).ok_or(Error::<T>::NonExistentAccount)?;
            let remainder = sender_balance
                .checked_sub(amount)
                .ok_or(Error::<T>::InsufficientBalance)?;

            let dest_old = Balances::<T>::get(&dest).unwrap_or(0);
            Balances::<T>::insert(&dest, dest_old.checked_add(amount).ok_or(Error::<T>::Overflow)?);
            Balances::<T>::insert(&sender, remainder);

            Self::deposit_event(Event::Transferred { from: sender, to: dest, amount });
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests;
