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

use merkle_mountain_range::{hash_with_index, Error, Hashable, MerkleMountainRange, VecStore};

type E = Vec<u8>;

#[test]
fn non_existing_node() {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);
    let mut size = 0;

    (0..=6u8).for_each(|i| {
        let n = vec![i];
        size = mmr.append(&n).unwrap();
    });

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
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);
    let mut size = 0;

    (0..=1u8).for_each(|i| {
        let n = vec![i];
        size = mmr.append(&n).unwrap();
    });

    let got = mmr.proof(2).unwrap().path[0];
    let got = format!("{}", got);

    assert_eq!("c3264807b084", got);
}

#[test]
fn verify_proof() {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);
    let mut size = 0;

    (0..6u8).for_each(|i| {
        let n = vec![i];
        size = mmr.append(&n).unwrap();
    });

    let proof = mmr.proof(4).unwrap();
    assert_eq!(3, proof.path.len());
}
