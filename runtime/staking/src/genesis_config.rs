// Copyright 2018 Chainpool.
//! Build a staking genesis block.

#![cfg(feature = "std")]

use rstd::prelude::*;
use codec::Encode;
use runtime_support::StorageValue;
use primitives::traits::As;
use primitives::{self, Perbill};
use super::{Trait, Intentions, CurrentEra, OfflineSlashGrace, MinimumValidatorCount,
	BondingDuration, SessionsPerEra, ValidatorCount, SessionReward, OfflineSlash,
	CurrentSessionReward, CurrentOfflineSlash};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GenesisConfig<T: Trait> {
	pub sessions_per_era: T::BlockNumber,
	pub current_era: T::BlockNumber,
	pub intentions: Vec<T::AccountId>,
	pub validator_count: u32,
	pub minimum_validator_count: u32,
	pub bonding_duration: T::BlockNumber,
	pub session_reward: Perbill,
	pub offline_slash: Perbill,
	pub current_session_reward: T::Balance,
	pub current_offline_slash: T::Balance,
	pub offline_slash_grace: u32,
}

impl<T: Trait> Default for GenesisConfig<T> {
	fn default() -> Self {
		GenesisConfig {
			sessions_per_era: T::BlockNumber::sa(1000),
			current_era: T::BlockNumber::sa(0),
			intentions: vec![],
			validator_count: 0,
			minimum_validator_count: 0,
			bonding_duration: T::BlockNumber::sa(1000),
			session_reward: Perbill::from_billionths(60),
			offline_slash: Perbill::from_fraction(0.001),
			current_session_reward: T::Balance::sa(0),
			current_offline_slash: T::Balance::sa(0),
			offline_slash_grace: 0,
		}
	}
}

impl<T: Trait> primitives::BuildStorage for GenesisConfig<T> {
	fn build_storage(self) -> ::std::result::Result<primitives::StorageMap, String> {
		Ok(map![
			Self::hash(<Intentions<T>>::key()).to_vec() => self.intentions.encode(),
			Self::hash(<SessionsPerEra<T>>::key()).to_vec() => self.sessions_per_era.encode(),
			Self::hash(<ValidatorCount<T>>::key()).to_vec() => self.validator_count.encode(),
			Self::hash(<MinimumValidatorCount<T>>::key()).to_vec() => self.minimum_validator_count.encode(),
			Self::hash(<BondingDuration<T>>::key()).to_vec() => self.bonding_duration.encode(),
			Self::hash(<CurrentEra<T>>::key()).to_vec() => self.current_era.encode(),
			Self::hash(<SessionReward<T>>::key()).to_vec() => self.session_reward.encode(),
			Self::hash(<OfflineSlash<T>>::key()).to_vec() => self.offline_slash.encode(),
			Self::hash(<CurrentSessionReward<T>>::key()).to_vec() => self.current_session_reward.encode(),
			Self::hash(<CurrentOfflineSlash<T>>::key()).to_vec() => self.current_offline_slash.encode(),
			Self::hash(<OfflineSlashGrace<T>>::key()).to_vec() => self.offline_slash_grace.encode()
		])
	}
}
