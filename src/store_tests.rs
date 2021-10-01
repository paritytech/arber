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

//! Merkle-Mountain-Range storage unit tests

use super::{Error, Store, VecStore};
use crate::Hashable;

#[test]
fn append_works() {
    #![allow(clippy::unit_cmp)]

    let elem = vec![0u8; 10];
    let h = elem.hash();

    let mut store = VecStore::<Vec<u8>>::new();
    let res = store.append(&elem, &[h]).unwrap();

    assert_eq!((), res);
    assert_eq!(elem, store.data.clone().unwrap()[0]);
    assert_eq!(h, store.hashes[0]);

    let elem = vec![1u8; 10];
    let h = elem.hash();

    let res = store.append(&elem, &[h]).unwrap();

    assert_eq!((), res);
    assert_eq!(elem, store.data.unwrap()[1]);
    assert_eq!(h, store.hashes[1]);
}

#[test]
fn peak_hash_at_works() {
    let mut store = VecStore::<Vec<u8>>::new();

    let elem = vec![0u8; 10];
    let h = elem.hash();
    let _ = store.append(&elem, &[h]);

    let elem = vec![1u8; 10];
    let h = elem.hash();
    let _ = store.append(&elem, &[h]);

    let peak = store.peak_hash_at(1).unwrap();

    assert_eq!(h, peak);
}

#[test]
fn peak_hash_at_fails() {
    let want = Err(Error::MissingHashAtIndex(3));

    let store = VecStore::<Vec<u8>>::new();
    let got = store.peak_hash_at(3);

    assert_eq!(want, got);
}

#[test]
fn hash_at_works() {
    let mut store = VecStore::<Vec<u8>>::new();

    let elem = vec![0u8; 10];
    let h = elem.hash();
    let _ = store.append(&elem, &[h]);

    let elem = vec![1u8; 10];
    let h = elem.hash();
    let _ = store.append(&elem, &[h]);

    let peak = store.hash_at(1).unwrap();

    assert_eq!(h, peak);
}

#[test]
fn hash_at_fails() {
    let want = Err(Error::MissingHashAtIndex(3));

    let store = VecStore::<Vec<u8>>::new();
    let got = store.hash_at(3);

    assert_eq!(want, got);
}
