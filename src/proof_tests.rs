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

//! Merkle Proof unit tests

use crate::{MerkleMountainRange, VecStore};

type E = Vec<u8>;

#[test]
fn minimal_proof_works() {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);

    let node = vec![42u8];
    let size = mmr.append(&node).unwrap();
    let proof = mmr.proof(size).unwrap();

    assert_eq!(proof.mmr_size, 1);
    assert_eq!(proof.path.len(), 0);

    let root = mmr.hash(size).unwrap();
    assert!(proof.verify(root, &node, size).unwrap());
}
