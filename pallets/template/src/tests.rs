use crate::{Balances, Error, Pallet, TotalIssuance};
use polkadot_sdk::{polkadot_sdk_frame::{self as frame, runtime::prelude::*, testing_prelude::*}, *};

pub type AccountId = u64;
pub type Block = frame::runtime::types_common::BlockOf<Test, ()>;

const ALICE: AccountId = 1;
const BOB: AccountId = 2;

parameter_types! {
	pub const Version: RuntimeVersion = RuntimeVersion {
		spec_name: std::borrow::Cow::Borrowed("test-runtime"),
		impl_name: std::borrow::Cow::Borrowed("test-runtime"),
		authoring_version: 1,
		spec_version: 0,
		impl_version: 0,
		apis: std::borrow::Cow::Borrowed(&[]),
		transaction_version: 1,
		system_version: 1,
	};
}

#[frame_construct_runtime]
mod runtime {
	#[runtime::runtime]
	#[runtime::derive(RuntimeCall, RuntimeEvent, RuntimeError, RuntimeOrigin, RuntimeTask)]
	pub struct Test;
	#[runtime::pallet_index(0)]
	pub type System = frame_system::Pallet<Test>;
	#[runtime::pallet_index(1)]
	pub type Template = crate::Pallet<Test>;
}



#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Test {
	type AccountId = self::AccountId;
	type Block = self::Block;
	type Version = self::Version;
	type Lookup = IdentityLookup<Self::AccountId>;
}

impl crate::Config for Test {}

fn new_test_ext() -> sp_io::TestExternalities {
	sp_io::TestExternalities::default()
}

#[test]
fn mint_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::<Test>::mint_unsafe(RuntimeOrigin::signed(ALICE), BOB, 1000));
		assert_eq!(Balances::<Test>::get(BOB), Some(1000));
		assert_eq!(TotalIssuance::<Test>::get(), Some(1000));
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::<Test>::mint_unsafe(RuntimeOrigin::signed(ALICE), ALICE, 500));
		assert_ok!(Pallet::<Test>::transfer(RuntimeOrigin::signed(ALICE), BOB, 100));
		assert_eq!(Balances::<Test>::get(ALICE), Some(400));
		assert_eq!(Balances::<Test>::get(BOB), Some(100));
		assert_eq!(TotalIssuance::<Test>::get(), Some(500));
	});
}

#[test]
fn transfer_insufficient_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::<Test>::mint_unsafe(RuntimeOrigin::signed(ALICE), ALICE, 50));
		assert_noop!(
			Pallet::<Test>::transfer(RuntimeOrigin::signed(ALICE), BOB, 100),
			Error::<Test>::InsufficientBalance
		);
		assert_eq!(Balances::<Test>::get(ALICE), Some(50));
	});
}

#[test]
fn transfer_to_self_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::<Test>::mint_unsafe(RuntimeOrigin::signed(ALICE), ALICE, 500));
		assert_noop!(
			Pallet::<Test>::transfer(RuntimeOrigin::signed(ALICE), ALICE, 100),
			Error::<Test>::TransferToSelf
		);
		assert_eq!(Balances::<Test>::get(ALICE), Some(500));
	});
}

#[test]
fn transfer_from_empty_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_noop!(
			Pallet::<Test>::transfer(RuntimeOrigin::signed(ALICE), BOB, 1),
			Error::<Test>::NonExistentAccount
		);
	});
}

#[test]
fn mint_overflow_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::<Test>::mint_unsafe(RuntimeOrigin::signed(ALICE), BOB, u128::MAX));
		assert_noop!(
			Pallet::<Test>::mint_unsafe(RuntimeOrigin::signed(ALICE), BOB, 1),
			Error::<Test>::Overflow
		);
	});
}
