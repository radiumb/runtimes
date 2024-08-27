// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `cumulus_pallet_dmp_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `zur1-vm-benchpas-001`, CPU: `AMD EPYC 9354 32-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./paseo.ah.local.raw.json")`, DB CACHE: 1024

// Executed Command:
// ./polkadot-parachain
// benchmark
// pallet
// --chain=./paseo.ah.local.raw.json
// --steps=50
// --repeat=20
// --pallet=cumulus_pallet_dmp_queue
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./ah-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `cumulus_pallet_dmp_queue`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> cumulus_pallet_dmp_queue::WeightInfo for WeightInfo<T> {
	/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::ServiceHead` (r:1 w:1)
	/// Proof: `MessageQueue::ServiceHead` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::Pages` (r:0 w:1)
	/// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
	fn on_idle_good_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65696`
		//  Estimated: `69161`
		// Minimum execution time: 112_439_000 picoseconds.
		Weight::from_parts(118_148_000, 0)
			.saturating_add(Weight::from_parts(0, 69161))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	fn on_idle_large_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65659`
		//  Estimated: `69124`
		// Minimum execution time: 67_883_000 picoseconds.
		Weight::from_parts(85_037_000, 0)
			.saturating_add(Weight::from_parts(0, 69124))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::ServiceHead` (r:1 w:1)
	/// Proof: `MessageQueue::ServiceHead` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::Pages` (r:0 w:1)
	/// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
	fn on_idle_overweight_good_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65726`
		//  Estimated: `69191`
		// Minimum execution time: 105_189_000 picoseconds.
		Weight::from_parts(110_317_000, 0)
			.saturating_add(Weight::from_parts(0, 69191))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	fn on_idle_overweight_large_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65689`
		//  Estimated: `69154`
		// Minimum execution time: 57_206_000 picoseconds.
		Weight::from_parts(61_574_000, 0)
			.saturating_add(Weight::from_parts(0, 69154))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
