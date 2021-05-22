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

//! Merkle proof store tests

use arber::{hash_with_index, Error, Hashable, MerkleMountainRange, MerkleProof, VecStore};
use codec::{DecodeAll, Encode};

type E = Vec<u8>;

fn make_mmr(num_leafs: u8) -> MerkleMountainRange<E, VecStore<E>> {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);

    (0..=num_leafs.saturating_sub(1)).for_each(|i| {
        let n = vec![i];
        let _ = mmr.append(&n).unwrap();
    });

    mmr
}

#[test]
fn non_existing_node() {
    let mmr = make_mmr(7);

    let want = Error::Proof("not a leaf node at pos 7".to_string());
    let res = mmr.proof(7);

    assert_eq!(Err(want), res);
}

#[test]
fn single_node() {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);

    let node = vec![42u8];
    let size = mmr.append(&node).unwrap();
    let proof = mmr.proof(size).unwrap();

    // this root hash is wrong because is has not been hashed with the node index
    let root = node.hash();

    let want = Error::Proof("root mismatch 8c058212512f != e00265169656".to_string());
    let got = proof.verify(root, &node, size);
    assert_eq!(Err(want), got);

    let root = hash_with_index(0, &root);
    assert!(proof.verify(root, &node, size).unwrap());
}

#[test]
fn minimal_mmr() {
    let mmr = make_mmr(2);
    let proof = mmr.proof(2).unwrap();

    assert!(proof
        .verify(mmr.hash(mmr.size).unwrap(), &vec![1u8], 2)
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
