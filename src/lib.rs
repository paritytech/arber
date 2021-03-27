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

use {std::marker::PhantomData, store::Store};

mod error;
mod store;
mod utils;

/// Merkle-Mountain-Range error codes
pub use error::Error;

/// Add a dummy hash type for now
pub struct Hash([u8; 32]);

pub struct MerkleMountainRange<'a, T, S>
where
    S: Store<T>,
{
    store: &'a mut S,
    _marker: PhantomData<T>,
}

impl<'a, T, S> MerkleMountainRange<'a, T, S>
where
    S: Store<T>,
{
    pub fn new(store: &'a mut S) -> Self {
        MerkleMountainRange {
            store,
            _marker: PhantomData,
        }
    }
}
