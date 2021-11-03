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

use codec::Encode;

use crate::{
    hash::ZERO_HASH, hash_with_index, Error, Hash, Hashable, MerkleMountainRange, VecStore,
};

type E = Vec<u8>;

fn make_mmr(num_leafs: u8) -> MerkleMountainRange<E, VecStore<E>> {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);

    (0..=num_leafs.saturating_sub(1)).for_each(|i| {
        let n = vec![i, 10];
        let _ = mmr.append(&n).unwrap();
    });

    mmr
}

#[test]
fn new_works() -> Result<(), Error> {
    let mmr = make_mmr(6);
    let hash = mmr.hash(5)?;

    // new MMR using a populated store
    let store = mmr.store;
    let mmr = MerkleMountainRange::<E, VecStore<E>>::new(mmr.size, store);

    assert_eq!(hash, mmr.hash(5)?);

    Ok(())
}

#[test]
fn append_two_nodes() -> Result<(), Error> {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);

    let n1 = vec![0u8, 10];
    let pos = mmr.append(&n1)?;

    assert_eq!(1, pos);

    let n2 = vec![1u8, 10];
    let pos = mmr.append(&n2)?;

    assert_eq!(3, pos);

    Ok(())
}

#[test]
fn append_tree_nodes() -> Result<(), Error> {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);

    let n1 = vec![0u8, 10];
    let pos = mmr.append(&n1)?;

    assert_eq!(1, pos);

    let n2 = vec![1u8, 10];
    let pos = mmr.append(&n2)?;

    assert_eq!(3, pos);

    let n3 = vec![2u8, 10];
    let pos = mmr.append(&n3)?;

    assert_eq!(4, pos);

    Ok(())
}

#[test]
fn validate_works() -> Result<(), Error> {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);

    // empty MMR is valid
    assert!(mmr.validate()?);

    let n1 = vec![0u8, 10];
    let mut size = mmr.append(&n1)?;

    assert_eq!(1, size);
    assert!(mmr.validate()?);

    let n2 = vec![1u8, 10];
    size = mmr.append(&n2)?;

    assert_eq!(3, size);
    assert!(mmr.validate()?);

    let n3 = vec![2u8, 10];
    size = mmr.append(&n3)?;

    assert_eq!(4, size);
    assert!(mmr.validate()?);

    Ok(())
}

#[test]
fn validate_fails() -> Result<(), Error> {
    let mut mmr = make_mmr(3);

    mmr.store.hashes[2] = Hash::from_hex("0x00")?;
    let got = mmr.validate().err().unwrap();

    // compare the actual error messages
    let want = "invalid node hash at idx 2: 000000000000 != 4f463b04ba9e".to_string();
    let got = format!("{}", got);

    assert_eq!(want, got);

    let mut mmr = make_mmr(7);

    mmr.store.hashes[6] = Hash::from_hex("0x00")?;
    let got = mmr.validate().err().unwrap();

    // compare the actual error messages
    let want = "invalid node hash at idx 6: 000000000000 != 36d109372a04".to_string();
    let got = format!("{}", got);

    assert_eq!(want, got);

    Ok(())
}

#[test]
fn proof_fails() {
    let mmr = make_mmr(2);

    assert_eq!(
        "expecting leaf node at pos: 3".to_string(),
        format!("{}", mmr.proof(3).err().unwrap()),
    );
}

#[test]
fn proof_works() -> Result<(), Error> {
    let mmr = make_mmr(2);
    let proof = mmr.proof(1)?;

    assert_eq!(3, proof.mmr_size);
    assert_eq!(1, proof.path.len());
    assert_eq!(mmr.hash(2)?, proof.path[0]);

    let mmr = make_mmr(4);
    let proof = mmr.proof(4)?;

    assert_eq!(7, proof.mmr_size);
    assert_eq!(2, proof.path.len());
    assert_eq!(mmr.hash(5)?, proof.path[0]);
    assert_eq!(mmr.hash(3)?, proof.path[1]);

    let mmr = make_mmr(11);
    let proof = mmr.proof(5)?;

    assert_eq!(19, proof.mmr_size);
    assert_eq!(4, proof.path.len());
    assert_eq!(mmr.hash(4)?, proof.path[0]);
    assert_eq!(mmr.hash(3)?, proof.path[1]);
    assert_eq!(mmr.hash(14)?, proof.path[2]);

    let h1 = mmr.hash(18)?;
    let h2 = mmr.hash(19)?;
    let h = (h1, h2).hash();
    let h = hash_with_index(mmr.size, &h);
    assert_eq!(h, proof.path[3]);

    Ok(())
}

#[test]
fn partial_prove_works() -> Result<(), Error> {
    let mut mmr = make_mmr(4);
    let proof_1 = mmr.proof(4)?;

    mmr.append(&vec![4; 10])?;
    mmr.append(&vec![5; 10])?;

    let proof_2 = mmr.partial_proof(4, 7)?;

    assert_eq!(proof_1, proof_2);

    let mut mmr = make_mmr(8);
    let proof_1 = mmr.proof(11)?;

    mmr.append(&vec![8; 10])?;
    mmr.append(&vec![9; 10])?;
    mmr.append(&vec![10; 10])?;

    let proof_2 = mmr.partial_proof(11, 15)?;

    assert_eq!(proof_1, proof_2);

    Ok(())
}

#[test]
fn bag_lower_peaks_works() -> Result<(), Error> {
    let mmr = make_mmr(2);
    let got = mmr.bag_lower_peaks(3);

    assert_eq!(None, got);

    let mmr = make_mmr(3);
    let want = mmr.hash(4)?;
    let got = mmr.bag_lower_peaks(3).unwrap();

    assert_eq!(want, got);

    let mmr = make_mmr(7);
    let h1 = mmr.hash(10)?;
    let h2 = mmr.hash(11)?;
    let want = (h1, h2).hash();
    let want = hash_with_index(mmr.size, &want);
    let got = mmr.bag_lower_peaks(7).unwrap();

    assert_eq!(want, got);

    Ok(())
}

#[test]
fn peak_path_works() -> Result<(), Error> {
    let mmr = make_mmr(2);
    let path = mmr.peak_path(3);

    assert!(path.is_empty());

    let mmr = make_mmr(3);
    let want = mmr.hash(4)?;
    let want = vec![want];
    let got = mmr.peak_path(3);

    assert_eq!(want, got);

    let want = mmr.hash(3)?;
    let want = vec![want];
    let got = mmr.peak_path(4);

    assert_eq!(want, got);

    let mmr = make_mmr(7);
    let h1 = mmr.hash(10)?;
    let h2 = mmr.hash(7)?;
    let want = vec![h1, h2];
    let got = mmr.peak_path(11);

    assert_eq!(want, got);

    let h1 = mmr.hash(11)?;
    let h2 = mmr.hash(7)?;
    let want = vec![h1, h2];
    let got = mmr.peak_path(10);

    assert_eq!(want, got);

    let h1 = mmr.hash(11)?;
    let h2 = mmr.hash(10)?;
    let want = (h2, h1).hash();
    let want = hash_with_index(mmr.size, &want);
    let want = vec![want];
    let got = mmr.peak_path(7);

    assert_eq!(want, got);

    Ok(())
}

#[test]
fn hash_error_works() {
    let s = VecStore::<E>::new();
    let mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);

    let want = Error::MissingHashAtIndex(0);
    let got = mmr.hash(0).err().unwrap();

    assert_eq!(want, got);

    let want = Error::MissingHashAtIndex(2);
    let got = mmr.hash(3).err().unwrap();

    assert_eq!(want, got);
}

#[test]
fn hash_works() -> Result<(), Error> {
    let mmr = make_mmr(3);

    let h1 = hash_with_index(0, &vec![0u8, 10].encode().hash());
    let h = mmr.hash(1)?;
    assert_eq!(h, h1);

    let h2 = hash_with_index(1, &vec![1u8, 10].encode().hash());
    let h = mmr.hash(2)?;
    assert_eq!(h, h2);

    let h3 = hash_with_index(2, &(h1, h2).hash());
    let h = mmr.hash(3)?;
    assert_eq!(h, h3);

    let h4 = hash_with_index(3, &vec![2u8, 10].encode().hash());
    let h = mmr.hash(4)?;
    assert_eq!(h, h4);

    let mmr = make_mmr(4);

    let h1 = hash_with_index(3, &vec![2u8, 10].encode().hash());
    let h = mmr.hash(4)?;
    assert_eq!(h, h1);

    let h2 = hash_with_index(4, &vec![3u8, 10].encode().hash());
    let h = mmr.hash(5)?;
    assert_eq!(h, h2);

    let h3 = hash_with_index(5, &(h1, h2).encode().hash());
    let h = mmr.hash(6)?;
    assert_eq!(h, h3);

    Ok(())
}

#[test]
fn peaks_works() -> Result<(), Error> {
    let mmr = make_mmr(1);
    let peaks = mmr.peaks()?;

    assert!(peaks.len() == 1);
    assert_eq!(mmr.hash(mmr.size)?, peaks[0]);

    let mmr = make_mmr(2);
    let peaks = mmr.peaks()?;

    assert!(peaks.len() == 1);
    assert_eq!(mmr.hash(mmr.size)?, peaks[0]);

    let mmr = make_mmr(4);
    let peaks = mmr.peaks()?;

    assert!(peaks.len() == 1);
    assert_eq!(mmr.hash(mmr.size)?, peaks[0]);

    let mmr = make_mmr(10);
    let peaks = mmr.peaks()?;

    assert!(peaks.len() == 2);
    assert_eq!(mmr.hash(15)?, peaks[0]);
    assert_eq!(mmr.hash(18)?, peaks[1]);

    let mmr = make_mmr(11);
    let peaks = mmr.peaks()?;

    assert!(peaks.len() == 3);
    assert_eq!(mmr.hash(15)?, peaks[0]);
    assert_eq!(mmr.hash(18)?, peaks[1]);
    assert_eq!(mmr.hash(19)?, peaks[2]);

    Ok(())
}

#[test]
fn root_works() -> Result<(), Error> {
    let mmr = make_mmr(1);
    let root = mmr.root()?;
    let hash = mmr.hash(1)?;

    assert_eq!(root, hash);

    let mmr = make_mmr(2);
    let root = mmr.root()?;
    let hash = mmr.hash(3)?;

    assert_eq!(root, hash);

    let mmr = make_mmr(4);
    let root = mmr.root()?;
    let hash = mmr.hash(7)?;

    assert_eq!(root, hash);

    let mmr = make_mmr(6);
    let root = mmr.root()?;
    let h1 = mmr.hash(7)?;
    let h2 = mmr.hash(10)?;
    let hash = hash_with_index(mmr.size, &(h1, h2).hash());

    assert_eq!(root, hash);

    let mmr = make_mmr(11);
    let root = mmr.root()?;
    let h1 = mmr.hash(18)?;
    let h2 = mmr.hash(19)?;
    let h2 = hash_with_index(mmr.size, &(h1, h2).hash());
    let h1 = mmr.hash(15)?;
    let hash = hash_with_index(mmr.size, &(h1, h2).hash());

    assert_eq!(root, hash);

    Ok(())
}

#[test]
fn root_fails() -> Result<(), Error> {
    let s = VecStore::<E>::new();
    let mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);
    let root = mmr.root()?;

    assert_eq!(ZERO_HASH, root);

    Ok(())
}
