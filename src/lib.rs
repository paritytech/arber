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
///
/// Again, positions are '1' based tree node positions, indices are '0' based `Store`
/// locations.
pub struct MerkleMountainRange<'a, T, S>
where
    T: Hashable + Clone,
    S: Store<T>,
{
    /// Total number of MMR nodes, i.e. MMR size
    pub size: u64,
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
            size: 0,
            store,
            _marker: PhantomData,
        }
    }

    /// Append `elem` to the MMR. Return new MMR size.
    pub fn append(&mut self, elem: &T) -> Result<u64, Error> {
        let idx = self.size;
        let node_hash = (idx, elem.clone()).hash();

        let (peak_map, node_height) = utils::peak_height_map(idx);

        // a new node always has to be a leave node (height = 0)
        if node_height != 0 {
            return Err(Error::Store(format!(
                "invalid leave height: {}",
                node_height
            )));
        }

        let (new, peak_hashes) = self.bag_the_peaks(node_hash, peak_map)?;

        self.store.append(elem, &peak_hashes)?;
        self.size += new;

        Ok(self.size)
    }

    /// Calculate single MMR root by 'bagging the peaks'.
    ///
    /// Return the number of new nodes added as well as a merkle path to the MMR root.
    fn bag_the_peaks(&self, node_hash: Hash, peak_map: u64) -> Result<(u64, Vec<Hash>), Error> {
        let mut idx = self.size;
        let mut new = 0;
        let mut height = 1u64;
        let mut merkle_path = vec![node_hash];
        let mut tmp_hash = node_hash;

        // at least one new node is added
        new += 1;

        while (peak_map & height) != 0 {
            let left_sibling = idx + 1 - 2 * height;
            let peak = self.store.peak_hash_at(left_sibling)?;

            tmp_hash = (peak, tmp_hash).hash();
            tmp_hash = (idx, tmp_hash).hash();
            merkle_path.push(tmp_hash);

            height *= 2;
            idx += 1;
            new += 1;
        }

        Ok((new, merkle_path))
    }
}

#[cfg(test)]
mod tests {
    use super::{MerkleMountainRange, VecStore};

    type E = Vec<u8>;

    #[test]
    fn append_two_nodes() {
        let mut s = VecStore::<E>::new();
        let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(&mut s);

        let n1 = vec![0u8, 10];
        let pos = mmr.append(&n1).unwrap();

        assert_eq!(1, pos);

        let n2 = vec![1u8, 10];
        let pos = mmr.append(&n2).unwrap();

        assert_eq!(3, pos);
    }

    #[test]
    fn append_tree_nodes() {
        let mut s = VecStore::<E>::new();
        let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(&mut s);

        let n1 = vec![0u8, 10];
        let pos = mmr.append(&n1).unwrap();

        assert_eq!(1, pos);

        let n2 = vec![1u8, 10];
        let pos = mmr.append(&n2).unwrap();

        assert_eq!(3, pos);

        let n3 = vec![2u8, 10];
        let pos = mmr.append(&n3).unwrap();

        assert_eq!(4, pos);
    }
}
