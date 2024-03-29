// Copyright (C) 2021 Parity Technologies (UK) Ltd.
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

use core::marker::PhantomData;

use codec::{Decode, Encode};

use crate::{
    hash::ZERO_HASH, hash_with_index, utils, Error, Hash, Hashable, MerkleProof, Result, Store,
};

#[cfg(test)]
#[path = "mmr_tests.rs"]
mod tests;

/// Merkle-Mountain-Range (MMR) implementation.
///
/// All tree positions start at `'1'`. MMR positions are depth-frist, post-order tree
/// traversal node positions and should not be seen as array indices.
///
/// The MMR `Store`, however, is a flat list representation of the MMR, i.e. an array of
/// nodes. Hence, `Store` elements are accessed using a `'0'` based index.
///
/// Again, positions are `'1'` based tree node positions, indices are `'0'` based `Store`
/// locations.
pub struct MerkleMountainRange<T, S>
where
    T: Clone + Decode + Encode,
    S: Store<T>,
{
    // total number of MMR nodes, i.e. MMR size
    size: u64,
    // backing store for the MMR
    store: S,
    // make rustc happy
    _marker: PhantomData<T>,
}

impl<T, S> MerkleMountainRange<T, S>
where
    T: Clone + Decode + Encode,
    S: Store<T>,
{
    pub fn new(size: u64, store: S) -> Self {
        MerkleMountainRange {
            size,
            store,
            _marker: PhantomData,
        }
    }

    /// Append `elem` to the MMR. Return new MMR size.
    pub fn append(&mut self, elem: &T) -> Result<u64> {
        let idx = self.size;
        let node_hash = hash_with_index(idx, &elem.encode().hash());

        let (peak_map, node_height) = utils::peak_height_map(idx);

        // a new node always has to be a leave node (height = 0)
        if node_height != 0 {
            return Err(Error::InvalidNodeHeight(node_height));
        }

        let (new, peak_hashes) = self.bag_the_peaks(node_hash, peak_map)?;

        self.store.append(elem, &peak_hashes)?;
        self.size += new;

        Ok(self.size)
    }

    /// Validate the MMR by re-calculating the hash of all inner, i.e. parent nodes.
    /// Retrun `true`, if the MMR is valid or an error.
    pub fn validate(&self) -> Result<bool> {
        for pos in 1..=self.size {
            let height = utils::node_height(pos.saturating_sub(1));

            // inner nodes, i.e. parents start at height 1
            if height > 0 {
                let idx = pos - 1u64;

                // recalculate parent hash
                let left_idx = idx - (1 << height);
                let left_hash = self.store.hash_at(left_idx)?;

                let right_idx = idx - 1;
                let right_hash = self.store.hash_at(right_idx)?;

                let tmp = (left_hash, right_hash).hash();
                let tmp = hash_with_index(idx, &tmp);

                // check against expected parent hash
                let parent_hash = self.store.hash_at(idx)?;

                if tmp != parent_hash {
                    return Err(Error::InvalidNodeHash(idx, parent_hash, tmp));
                }
            }
        }

        Ok(true)
    }

    /// Return a MMR membership proof for a leaf node at position `pos`.
    ///
    /// Note that this proof is `complete` in the sense that it does include all
    /// nodes up to the MMR root.
    ///
    /// See [`partial_proof()`] for a proof containing only a subset of the nodes.
    ///
    pub fn proof(&self, pos: u64) -> Result<MerkleProof> {
        if !utils::is_leaf(pos.saturating_sub(1)) {
            return Err(Error::ExpectingLeafNode(pos));
        }

        self.hash(pos)?;

        let family_path = utils::family_path(pos, self.size);

        let mut path = family_path
            .iter()
            .filter_map(|x| self.hash(x.1).ok())
            .collect::<Vec<_>>();

        let peak = if let Some(n) = family_path.last() {
            n.0
        } else {
            pos
        };

        path.append(&mut self.peak_path(peak));

        Ok(MerkleProof {
            mmr_size: self.size,
            path,
        })
    }

    /// Return a MMR membership proof for a leaf at position `pos` including `size` nodes.
    ///
    /// Note that this is a `partial` proof in the sense that it does **not** include all
    /// nodes. Only a subset of nodes for a MMR with the given size is included.
    ///
    /// See [`proof()`] for a complete proof.
    ///
    pub fn partial_proof(&self, pos: u64, size: u64) -> Result<MerkleProof> {
        if !utils::is_leaf(pos) {
            return Err(Error::ExpectingLeafNode(pos));
        }

        self.hash(pos)?;

        let family_path = utils::family_path(pos, size);

        let mut path = family_path
            .iter()
            .filter_map(|x| self.hash(x.1).ok())
            .collect::<Vec<_>>();

        let peak = if let Some(n) = family_path.last() {
            n.0
        } else {
            pos
        };

        if peak < size {
            path.append(&mut self.peak_path(peak));
        }

        Ok(MerkleProof {
            mmr_size: size,
            path,
        })
    }

    /// Return node hash at `pos`.
    ///
    /// Note that in case of an error, [`Error::Store`] is returned and the error
    /// message is referring to `pss - 1`, i.e. an index.
    pub fn hash(&self, pos: u64) -> Result<Hash> {
        self.store.hash_at(pos.saturating_sub(1))
    }

    /// Return MMR peak hashes as a vec
    ///
    /// Peaks are listed left to right, starting with the leftmost peak. The leftmost
    /// peak is also always the 'highest' peak.
    pub fn peaks(&self) -> Result<Vec<Hash>> {
        let peaks = utils::peaks(self.size);

        let mut hashes = Vec::new();

        for p in peaks {
            hashes.push(self.store.hash_at(p.saturating_sub(1))?);
        }

        Ok(hashes)
    }

    /// Return the root hash of the MMR.
    ///
    /// Find all the current peaks and bag them together into a single peak hash.
    pub fn root(&self) -> Result<Hash> {
        if self.size == 0 {
            return Ok(ZERO_HASH);
        }

        let mut hash = None;
        let peaks = self.peaks()?;

        for p in peaks.into_iter().rev() {
            hash = match hash {
                None => Some(p),
                Some(h) => Some(hash_with_index(self.size, &(p, h).hash())),
            }
        }

        hash.ok_or(Error::MissingRootNode)
    }

    /// Return MMR size, i.e. total number of nodes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Calculate a single MMR root by 'bagging the peaks'.
    ///
    /// Return the number of new nodes added as well as a merkle path to the MMR root.
    ///
    /// `peak_map` is obtained from  [`utils::peak_height_map`] and contains an encoded
    /// list of the height for already exisiting MMR peaks.
    fn bag_the_peaks(&self, node_hash: Hash, peak_map: u64) -> Result<(u64, Vec<Hash>)> {
        // start with the node added before `node_hash`
        let mut idx = self.size;
        // number of new nodes added while bagging
        let mut new = 0;
        // current height encoded as power of 2
        let mut height = 1u64;
        // merkle path to the root, always starts with the new node
        let mut merkle_path = vec![node_hash];
        // init peak hash with new node
        let mut peak_hash = node_hash;

        new += 1; // we add at least `node_hash`

        while (peak_map & height) != 0 {
            let left_idx = idx + 1 - 2 * height;
            let left_hash = self.store.hash_at(left_idx)?;

            idx += 1; // idx for new peak

            peak_hash = (left_hash, peak_hash).hash();
            peak_hash = hash_with_index(idx, &peak_hash);
            merkle_path.push(peak_hash);

            height *= 2; // next power of 2
            new += 1; // new peak added
        }

        Ok((new, merkle_path))
    }

    /// Path with all peak hashes excluding the peak at `pos`.
    ///
    /// The returned path vector will contain the peak hashes from rigth to left,
    /// i.e. from the lowest to the highest peak.
    fn peak_path(&self, pos: u64) -> Vec<Hash> {
        let lower = self.bag_lower_peaks(pos);

        // path with higher peaks, if there are any
        let mut path = utils::peaks(self.size)
            .into_iter()
            .filter(|&n| n < pos)
            .filter_map(|n| self.hash(n).ok())
            .collect::<Vec<_>>();

        if let Some(lower) = lower {
            path.push(lower);
        }

        path.reverse();

        path
    }

    /// Bag all the peaks 'lower' than the peak at `pos`.
    ///
    /// Peaks are ordered left to right. The leftmost peak is always the 'highest' peak.
    /// Due to this oredering, a 'lower' peak will always have a **higher** index.
    fn bag_lower_peaks(&self, pos: u64) -> Option<Hash> {
        let peaks = utils::peaks(self.size)
            .into_iter()
            .filter(|&x| x > pos)
            .filter_map(|x| self.hash(x).ok());

        let mut hash = None;

        peaks.rev().for_each(|peak| {
            hash = match hash {
                None => Some(peak),
                Some(hash) => {
                    let h = (peak, hash).hash();
                    Some(hash_with_index(self.size, &h))
                }
            }
        });

        hash
    }
}
