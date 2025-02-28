// Copyright 2017-2020 Parity Technologies (UK) Ltd.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn claim() -> Weight;
    fn mint_claim() -> Weight;
    fn claim_attest() -> Weight;
    fn attest() -> Weight;
    fn move_claim() -> Weight;
    fn set_vesting() -> Weight;
}

pub struct TestWeightInfo;
impl WeightInfo for TestWeightInfo {
    fn claim() -> Weight { 0 }
    fn mint_claim() -> Weight { 0 }
    fn claim_attest() -> Weight { 0 }
    fn attest() -> Weight { 0 }
    fn move_claim() -> Weight { 0 }
    fn set_vesting() -> Weight { 0 }
}

/// Weight functions for claims.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn claim() -> Weight {
        (466_905_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(7 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
    }
    fn mint_claim() -> Weight {
        (19_003_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn claim_attest() -> Weight {
        (471_915_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(7 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
    }
    fn attest() -> Weight {
        (156_649_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(8 as Weight))
            .saturating_add(T::DbWeight::get().writes(8 as Weight))
    }
    fn move_claim() -> Weight {
        (39_612_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
    }
    fn set_vesting() -> Weight {
        (24_376_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}
