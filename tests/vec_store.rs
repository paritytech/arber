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

//! MMR vector store tests

use arber::{MutableMerkleMountainRange, VecStore};

type E = Vec<u8>;

#[test]
fn append_two_nodes() {
    let s = VecStore::<E>::new();
    let mut mmr = MutableMerkleMountainRange::<E, VecStore<E>>::new(0, s);

    let n1 = vec![0u8, 10];
    let pos = mmr.append(&n1).unwrap();

    assert_eq!(1, pos);

    let n2 = vec![1u8, 10];
    let pos = mmr.append(&n2).unwrap();

    assert_eq!(3, pos);
}

#[test]
fn append_multiple_nodes() {
    let s = VecStore::<E>::new();
    let mut mmr = MutableMerkleMountainRange::<E, VecStore<E>>::new(0, s);
    let mut size = 0;

    (0..=10u8).for_each(|i| {
        let n = vec![i, 10];
        size = mmr.append(&n).unwrap();
    });

    assert_eq!(19, size);
}

#[test]
fn validate_works() {
    let mut s = VecStore::<E>::new();
    let mut mmr = MutableMerkleMountainRange::<E, VecStore<E>>::new(0, s);
    let mut size = 0;

    (0..=2u8).for_each(|i| {
        let n = vec![i, 10];
        size = mmr.append(&n).unwrap();
    });

    assert_eq!(4, size);
    assert!(mmr.validate().unwrap());

    s = VecStore::<E>::new();
    mmr = MutableMerkleMountainRange::<E, VecStore<E>>::new(0, s);
    size = 0;

    (0..=6u8).for_each(|i| {
        let n = vec![i, 10];
        size = mmr.append(&n).unwrap();
    });

    assert_eq!(11, size);
    assert!(mmr.validate().unwrap());

    s = VecStore::<E>::new();
    mmr = MutableMerkleMountainRange::<E, VecStore<E>>::new(0, s);
    size = 0;

    (0..=10u8).for_each(|i| {
        let n = vec![i, 10];
        size = mmr.append(&n).unwrap();
    });

    assert_eq!(19, size);
    assert!(mmr.validate().unwrap());
}
