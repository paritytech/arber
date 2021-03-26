// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Merkle-Mountain-Range implementation.

#![cfg_attr(not(feature = "std"), no_std)]

use {sp_std::marker::PhantomData, store::Store};

mod error;
mod store;
mod utils;

/// Merkle-Mountain-Range error codes
pub use error::Error;

pub struct MerkleMountainRange<'a, T, B>
where
    T: Clone,
    B: Store<T>,
{
    store: &'a mut B,
    _marker: PhantomData<T>,
}

impl<'a, T, B> MerkleMountainRange<'a, T, B>
where
    T: Clone,
    B: Store<T>,
{
    pub fn new(store: &'a mut B) -> Self {
        MerkleMountainRange {
            store,
            _marker: PhantomData,
        }
    }
}
