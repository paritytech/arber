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
fn peak_hash_works() {
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
fn peak_hash_fails() {
    let want = Err(Error::MissingHashAtIndex(3));

    let store = VecStore::<Vec<u8>>::new();
    let got = store.hash_at(3);

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
