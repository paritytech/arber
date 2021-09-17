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
