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

use hash::ZERO_HASH;
use utils::is_leaf;

pub use {
    error::Error,
    hash::{hash_with_index, Hash, Hashable},
    proof::MerkleProof,
    store::{Store, VecStore},
};

mod error;
mod hash;
mod proof;
mod store;
mod utils;

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
pub struct MerkleMountainRange<T, S>
where
    T: Hashable + Clone,
    S: Store<T>,
{
    /// Total number of MMR nodes, i.e. MMR size
    pub size: u64,
    // backing store for the MMR
    store: S,
    // make rustc happy
    _marker: PhantomData<T>,
}

impl<'a, T, S> MerkleMountainRange<T, S>
where
    T: Hashable + Clone,
    S: Store<T>,
{
    pub fn new(store: S) -> Self {
        MerkleMountainRange {
            size: 0,
            store,
            _marker: PhantomData,
        }
    }

    /// Append `elem` to the MMR. Return new MMR size.
    pub fn append(&mut self, elem: &T) -> Result<u64, Error> {
        let idx = self.size;
        let node_hash = hash_with_index(idx, &elem.clone().hash());

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

    /// Validate the MMR by re-calculating the hash of all inner, i.e. parent nodes.
    /// Retrun `true`, if the MMR is valid or an error.
    pub fn validate(&self) -> Result<bool, Error> {
        for pos in 1..=self.size {
            let height = utils::node_height(pos);

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
                    return Err(Error::Validate(format!(
                        "idx {}: {} != {}",
                        idx, parent_hash, tmp
                    )));
                }
            }
        }

        Ok(true)
    }

    /// Return a MMR membership proof for a leaf node at position `pos`.
    pub fn proof(&self, pos: u64) -> Result<MerkleProof, Error> {
        if !is_leaf(pos) {
            return Err(Error::Proof(format!("not a leaf node at pos {}", pos)));
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

    /// Return node hash at `pos`.
    ///
    /// Note that in case of an error, [`Error::Store`] is returned and the error
    /// message is referring to `pss - 1`, i.e. an index.
    pub fn hash(&self, pos: u64) -> Result<Hash, Error> {
        self.store.hash_at(pos.saturating_sub(1))
    }

    /// Return MMR peaks as a vec
    pub fn peaks(&self) -> Vec<Hash> {
        utils::peaks(self.size)
            .into_iter()
            .filter_map(move |p| self.store.peak_hash_at(p.saturating_sub(1)).ok())
            .collect()
    }

    /// Return the root hash of the MMR.
    ///
    /// Find all the current peaks and bag them together into a single peak hash.
    pub fn root(&self) -> Result<Hash, Error> {
        if self.size == 0 {
            return Ok(ZERO_HASH);
        }

        let mut hash = None;
        let preaks = self.peaks();

        for p in preaks.into_iter().rev() {
            hash = match hash {
                None => Some(p),
                Some(h) => Some(hash_with_index(self.size, &(p, h).hash())),
            }
        }

        hash.ok_or_else(|| Error::Invalid(format!("root missing")))
    }

    /// Calculate a single MMR root by 'bagging the peaks'.
    ///
    /// Return the number of new nodes added as well as a merkle path to the MMR root.
    ///
    /// `peak_map` is obtained from  [`utils::peak_height_map`] and contains an encoded
    /// list of the height for already exisiting MMR peaks.
    fn bag_the_peaks(&self, node_hash: Hash, peak_map: u64) -> Result<(u64, Vec<Hash>), Error> {
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
            let left_hash = self.store.peak_hash_at(left_idx)?;

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

#[cfg(test)]
mod tests {
    use crate::Hashable;

    use super::{hash_with_index, Error, Hash, MerkleMountainRange, VecStore};

    type E = Vec<u8>;

    fn make_mmr(num_leafs: u8) -> MerkleMountainRange<E, VecStore<E>> {
        let s = VecStore::<E>::new();
        let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);

        (0..=num_leafs.saturating_sub(1)).for_each(|i| {
            let n = vec![i, 10];
            let _ = mmr.append(&n).unwrap();
        });

        mmr
    }

    #[test]
    fn append_two_nodes() {
        let s = VecStore::<E>::new();
        let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);

        let n1 = vec![0u8, 10];
        let pos = mmr.append(&n1).unwrap();

        assert_eq!(1, pos);

        let n2 = vec![1u8, 10];
        let pos = mmr.append(&n2).unwrap();

        assert_eq!(3, pos);
    }

    #[test]
    fn append_tree_nodes() {
        let s = VecStore::<E>::new();
        let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);

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

    #[test]
    fn validate_works() {
        let s = VecStore::<E>::new();
        let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);

        // empty MMR is valid
        assert!(mmr.validate().unwrap());

        let n1 = vec![0u8, 10];
        let mut size = mmr.append(&n1).unwrap();

        assert_eq!(1, size);
        assert!(mmr.validate().unwrap());

        let n2 = vec![1u8, 10];
        size = mmr.append(&n2).unwrap();

        assert_eq!(3, size);
        assert!(mmr.validate().unwrap());

        let n3 = vec![2u8, 10];
        size = mmr.append(&n3).unwrap();

        assert_eq!(4, size);
        assert!(mmr.validate().unwrap());
    }

    #[test]
    fn validate_fails() {
        let mut mmr = make_mmr(3);

        let want = Error::Validate("idx 2: 000000000000 != 9f7d5dc4ed82".to_string());

        mmr.store.hashes[2] = Hash::from_hex("0x00").unwrap();
        let got = mmr.validate().err().unwrap();

        assert_eq!(want, got);

        let mut mmr = make_mmr(7);

        let want = Error::Validate("idx 6: 000000000000 != 2cabe06f9728".to_string());

        mmr.store.hashes[6] = Hash::from_hex("0x00").unwrap();
        let got = mmr.validate().err().unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn proof_fails() {
        let mmr = make_mmr(2);

        let want = Error::Proof("not a leaf node at pos 3".to_string());
        let got = mmr.proof(3).err().unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn proof_works() {
        let mmr = make_mmr(2);
        let proof = mmr.proof(1).unwrap();

        assert_eq!(3, proof.mmr_size);
        assert_eq!(1, proof.path.len());
        assert_eq!(mmr.hash(2).unwrap(), proof.path[0]);

        let mmr = make_mmr(4);
        let proof = mmr.proof(4).unwrap();

        assert_eq!(7, proof.mmr_size);
        assert_eq!(2, proof.path.len());
        assert_eq!(mmr.hash(5).unwrap(), proof.path[0]);
        assert_eq!(mmr.hash(3).unwrap(), proof.path[1]);

        let mmr = make_mmr(11);
        let proof = mmr.proof(5).unwrap();

        assert_eq!(19, proof.mmr_size);
        assert_eq!(4, proof.path.len());
        assert_eq!(mmr.hash(4).unwrap(), proof.path[0]);
        assert_eq!(mmr.hash(3).unwrap(), proof.path[1]);
        assert_eq!(mmr.hash(14).unwrap(), proof.path[2]);

        let h1 = mmr.hash(18).unwrap();
        let h2 = mmr.hash(19).unwrap();
        let h = (h1, h2).hash();
        let h = hash_with_index(mmr.size, &h);
        assert_eq!(h, proof.path[3]);
    }

    #[test]
    fn bag_lower_peaks_works() {
        let mmr = make_mmr(2);
        let got = mmr.bag_lower_peaks(3);

        assert_eq!(None, got);

        let mmr = make_mmr(3);
        let want = mmr.hash(4).unwrap();
        let got = mmr.bag_lower_peaks(3).unwrap();

        assert_eq!(want, got);

        let mmr = make_mmr(7);
        let h1 = mmr.hash(10).unwrap();
        let h2 = mmr.hash(11).unwrap();
        let want = (h1, h2).hash();
        let want = hash_with_index(mmr.size, &want);
        let got = mmr.bag_lower_peaks(7).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn peak_path_works() {
        let mmr = make_mmr(2);
        let path = mmr.peak_path(3);

        assert!(path.is_empty());

        let mmr = make_mmr(3);
        let want = mmr.hash(4).unwrap();
        let want = vec![want];
        let got = mmr.peak_path(3);

        assert_eq!(want, got);

        let want = mmr.hash(3).unwrap();
        let want = vec![want];
        let got = mmr.peak_path(4);

        assert_eq!(want, got);

        let mmr = make_mmr(7);
        let h1 = mmr.hash(10).unwrap();
        let h2 = mmr.hash(7).unwrap();
        let want = vec![h1, h2];
        let got = mmr.peak_path(11);

        assert_eq!(want, got);

        let h1 = mmr.hash(11).unwrap();
        let h2 = mmr.hash(7).unwrap();
        let want = vec![h1, h2];
        let got = mmr.peak_path(10);

        assert_eq!(want, got);

        let h1 = mmr.hash(11).unwrap();
        let h2 = mmr.hash(10).unwrap();
        let want = (h2, h1).hash();
        let want = hash_with_index(mmr.size, &want);
        let want = vec![want];
        let got = mmr.peak_path(7);

        assert_eq!(want, got);
    }

    #[test]
    fn hash_error_works() {
        let s = VecStore::<E>::new();
        let mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);

        let want = Error::Store("missing hash at: 0".to_string());
        let got = mmr.hash(0).err().unwrap();

        assert_eq!(want, got);

        let want = Error::Store("missing hash at: 2".to_string());
        let got = mmr.hash(3).err().unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn hash_works() {
        let mmr = make_mmr(3);

        let h1 = hash_with_index(0, &vec![0u8, 10].hash());
        let h = mmr.hash(1).unwrap();
        assert_eq!(h, h1);

        let h2 = hash_with_index(1, &vec![1u8, 10].hash());
        let h = mmr.hash(2).unwrap();
        assert_eq!(h, h2);

        let h3 = hash_with_index(2, &(h1, h2).hash());
        let h = mmr.hash(3).unwrap();
        assert_eq!(h, h3);

        let h4 = hash_with_index(3, &vec![2u8, 10].hash());
        let h = mmr.hash(4).unwrap();
        assert_eq!(h, h4);

        let mmr = make_mmr(4);

        let h1 = hash_with_index(3, &vec![2u8, 10].hash());
        let h = mmr.hash(4).unwrap();
        assert_eq!(h, h1);

        let h2 = hash_with_index(4, &vec![3u8, 10].hash());
        let h = mmr.hash(5).unwrap();
        assert_eq!(h, h2);

        let h3 = hash_with_index(5, &(h1, h2).hash());
        let h = mmr.hash(6).unwrap();
        assert_eq!(h, h3);
    }

    #[test]
    fn peaks_works() {
        let mmr = make_mmr(1);
        let peaks = mmr.peaks();

        assert!(peaks.len() == 1);
        assert_eq!(mmr.hash(mmr.size).unwrap(), peaks[0]);

        let mmr = make_mmr(2);
        let peaks = mmr.peaks();

        assert!(peaks.len() == 1);
        assert_eq!(mmr.hash(mmr.size).unwrap(), peaks[0]);

        let mmr = make_mmr(4);
        let peaks = mmr.peaks();

        assert!(peaks.len() == 1);
        assert_eq!(mmr.hash(mmr.size).unwrap(), peaks[0]);

        let mmr = make_mmr(10);
        let peaks = mmr.peaks();

        assert!(peaks.len() == 2);
        assert_eq!(mmr.hash(15).unwrap(), peaks[0]);
        assert_eq!(mmr.hash(18).unwrap(), peaks[1]);

        let mmr = make_mmr(11);
        let peaks = mmr.peaks();

        assert!(peaks.len() == 3);
        assert_eq!(mmr.hash(15).unwrap(), peaks[0]);
        assert_eq!(mmr.hash(18).unwrap(), peaks[1]);
        assert_eq!(mmr.hash(19).unwrap(), peaks[2]);
    }
}
