use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;

/*
	TODO: Combine all generic types and their trait bounds into a single `pub trait Config`.
	When you are done, your `Pallet` can simply be defined with `Pallet<T: Config>`.
*/

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

/*
	TODO: Update all of these functions to use your new configuration trait.
*/

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = *self.nonce.get(&who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	/*
		TODO: Create a `struct TestConfig`, and implement `super::Config` on it with concrete types.
		Use this struct to instantiate your `Pallet`.
	*/
	struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let mut system = super::Pallet::<TestConfig>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1));
		assert_eq!(system.nonce.get(&"bob".to_string()), None);
	}
}
