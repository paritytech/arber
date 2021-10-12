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

//! Merkle proof store tests

use arber::{hash_with_index, Hashable, MerkleMountainRange, MerkleProof, VecStore};
use codec::{DecodeAll, Encode};

type E = Vec<u8>;

fn make_mmr(num_leafs: u8) -> MerkleMountainRange<E, VecStore<E>> {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);

    (0..=num_leafs.saturating_sub(1)).for_each(|i| {
        let n = vec![i];
        let _ = mmr.append(&n).unwrap();
    });

    mmr
}

#[test]
fn non_existing_node() {
    let mmr = make_mmr(7);

    assert_eq!(
        "expecting leaf node at pos: 7".to_string(),
        format!("{}", mmr.proof(7).err().unwrap()),
    );
}

#[test]
fn single_node() {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);

    let node = vec![42u8];
    let size = mmr.append(&node).unwrap();
    let proof = mmr.proof(size).unwrap();

    // this root hash is wrong because is has not been hashed with the node index
    let root = node.hash();

    assert_eq!(
        "invalid root hash: 8c058212512f != e00265169656".to_string(),
        format!("{}", proof.verify(root, &node, size).err().unwrap())
    );

    let root = hash_with_index(0, &root);
    assert!(proof.verify(root, &node, size).unwrap());
}

#[test]
fn minimal_mmr() {
    let mmr = make_mmr(2);
    let proof = mmr.proof(2).unwrap();

    assert!(proof
        .verify(mmr.hash(mmr.size()).unwrap(), &vec![1u8], 2)
        .unwrap());
}

#[test]
fn verify_proof_single_peak() {
    let mmr = make_mmr(4);
    let proof = mmr.proof(5).unwrap();

    assert_eq!(7, proof.mmr_size);
    assert_eq!(2, proof.path.len());

    assert!(proof.verify(mmr.root().unwrap(), &vec![3u8], 5).unwrap());
}

#[test]
fn verify_proof_two_peaks() {
    let mmr = make_mmr(6);
    let proof = mmr.proof(8).unwrap();

    assert_eq!(10, proof.mmr_size);
    assert_eq!(2, proof.path.len());
    assert!(proof.verify(mmr.root().unwrap(), &vec![4u8], 8).unwrap());
}

#[test]
fn verify_proof_three_peaks() {
    let mmr = make_mmr(11);

    let proof = mmr.proof(5).unwrap();
    assert!(proof.verify(mmr.root().unwrap(), &vec![3u8], 5).unwrap());

    let proof = mmr.proof(16).unwrap();
    assert!(proof.verify(mmr.root().unwrap(), &vec![8u8], 16).unwrap());

    let proof = mmr.proof(19).unwrap();
    assert!(proof.verify(mmr.root().unwrap(), &vec![10u8], 19).unwrap());
}

#[test]
fn proof_encode_decode() {
    let mmr = make_mmr(11);
    let proof = mmr.proof(5).unwrap();
    let bytes = proof.encode();
    let proof = MerkleProof::decode_all(&bytes).unwrap();

    assert!(proof.verify(mmr.root().unwrap(), &vec![3u8], 5).unwrap());
}
