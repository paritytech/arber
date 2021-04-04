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

use std::marker::PhantomData;

mod error;
mod hash;
mod store;
mod utils;

pub use error::Error;
pub use hash::{Hash, Hashable};
pub use store::{Store, VecStore};

/// Merkle Mountain Range (MMR) implementation.
///
/// All tree positions start at '1'. MMR positions are depth-frist, post-order tree
/// traversal node positions and should not be seen as array indices.
///
/// The MMR `Store`, however, is a flat list representation of the MMR, i.e. an array of
/// nodes. Hence, `Store` elements are accessed using a '0' based index.
pub struct MerkleMountainRange<'a, T, S>
where
    T: Hashable + Clone,
    S: Store<T>,
{
    /// Last position within the MMR
    pub last_pos: u64,
    // backing store for the MMR
    store: &'a mut S,
    // make rustc happy
    _marker: PhantomData<T>,
}

impl<'a, T, S> MerkleMountainRange<'a, T, S>
where
    T: Hashable + Clone,
    S: Store<T>,
{
    pub fn new(store: &'a mut S) -> Self {
        MerkleMountainRange {
            last_pos: 0,
            store,
            _marker: PhantomData,
        }
    }

    /// Append `elem` to the MMR
    pub fn append(&mut self, elem: &T) -> Result<u64, Error> {
        let idx = self.last_pos;
        let node_hash = (idx, elem).hash();
        let hashes = vec![node_hash];

        self.store.append(elem, &hashes)?;
        self.last_pos += 1;
        Ok(self.last_pos)
    }
}
