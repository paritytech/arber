// Copyright (C) 2021 Andreas Doerr
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Merkle-Mountain-Range implementation unit tests

use crate::{hash::ZERO_HASH, Hashable};

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

#[test]
fn root_works() {
    let mmr = make_mmr(1);
    let root = mmr.root().unwrap();
    let hash = mmr.hash(1).unwrap();

    assert_eq!(root, hash);

    let mmr = make_mmr(2);
    let root = mmr.root().unwrap();
    let hash = mmr.hash(3).unwrap();

    assert_eq!(root, hash);

    let mmr = make_mmr(4);
    let root = mmr.root().unwrap();
    let hash = mmr.hash(7).unwrap();

    assert_eq!(root, hash);

    let mmr = make_mmr(6);
    let root = mmr.root().unwrap();
    let h1 = mmr.hash(7).unwrap();
    let h2 = mmr.hash(10).unwrap();
    let hash = hash_with_index(mmr.size, &(h1, h2).hash());

    assert_eq!(root, hash);

    let mmr = make_mmr(11);
    let root = mmr.root().unwrap();
    let h1 = mmr.hash(18).unwrap();
    let h2 = mmr.hash(19).unwrap();
    let h2 = hash_with_index(mmr.size, &(h1, h2).hash());
    let h1 = mmr.hash(15).unwrap();
    let hash = hash_with_index(mmr.size, &(h1, h2).hash());

    assert_eq!(root, hash);
}

#[test]
fn root_fails() {
    let s = VecStore::<E>::new();
    let mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);
    let root = mmr.root().unwrap();

    assert_eq!(ZERO_HASH, root);
}
