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

//! Hash type unit tests

use super::{hash_with_index, Error, Hash, Hashable};

macro_rules! hash_two {
    ($a:expr, $b:expr) => {{
        use blake2::{Blake2b, Digest};

        let mut h = Blake2b::new();
        h.update($a);
        h.update($b);
        let v = h.finalize();
        Hash::from_vec(&v)
    }};
}

#[test]
fn from_vec_works() {
    let v = vec![1, 2, 3];
    let h = format!("{}", Hash::from_vec(&v));
    assert_eq!(h, "010203000000");

    let v = Vec::new();
    let h = format!("{}", Hash::from_vec(&v));
    assert_eq!(h, "000000000000");

    let v = vec![222, 173, 202, 254, 186, 190];
    let h = format!("{}", Hash::from_vec(&v));
    assert_eq!(h, "deadcafebabe");
}

#[test]
fn from_hex_works() {
    let want = Hash::from_vec(&[]);
    let got = Hash::from_hex("0x00").unwrap();
    assert_eq!(want, got);

    let want = Hash::from_vec(&[202, 254]);
    let got = Hash::from_hex("0xcafe").unwrap();
    assert_eq!(want, got);

    let want = Hash::from_vec(&[222, 173, 202, 254, 186, 190]);
    let got = Hash::from_hex("0xdeadcafebabe").unwrap();
    assert_eq!(want, got);
}

#[test]
fn from_hex_error() {
    let want = Error::InvalidHexString("000".to_string());
    let got = Hash::from_hex("0x000").err().unwrap();
    assert_eq!(want, got);

    let want = Error::InvalidHexString("thisisbad".to_string());
    let got = Hash::from_hex("0xthisisbad").err().unwrap();
    assert_eq!(want, got);
}

#[test]
fn vec_hash_works() {
    let v1: Vec<u8> = vec![0, 0, 0, 0];
    let h1 = v1.hash();

    let v2: Vec<u8> = vec![0, 0, 0, 0];
    let h2 = v2.hash();

    let v3: Vec<u8> = vec![0, 0, 0, 1];
    let h3 = v3.hash();

    assert_eq!(h1, h2);
    assert_ne!(h1, h3);
    assert_ne!(h2, h3);
}

#[test]
fn u64_hash_works() {
    let u1 = 0u64;
    let h1 = u1.hash();

    let u2 = 0u64;
    let h2 = u2.hash();

    let u3 = 1u64;
    let h3 = u3.hash();

    assert_eq!(h1, h2);
    assert_ne!(h1, h3);
    assert_ne!(h2, h3);
}
#[test]
fn u32_hash_works() {
    let u1 = 0u32;
    let h1 = u1.hash();

    let u2 = 0u32;
    let h2 = u2.hash();

    let u3 = 1u32;
    let h3 = u3.hash();

    assert_eq!(h1, h2);
    assert_ne!(h1, h3);
    assert_ne!(h2, h3);
}

#[test]
fn tuple_hash_works() {
    let h1 = (1u64, vec![0u8; 10]).hash();
    let h2 = (1u64, vec![0u8; 10]).hash();
    let h3 = (2u64, vec![0u8; 10]).hash();

    assert_eq!(h1, h2);
    assert_ne!(h1, h3);
    assert_ne!(h2, h3);
}

#[test]
fn hash_two_works() {
    let h1 = hash_two!(1u64.to_le_bytes(), vec![0u8; 10]);
    let h2 = (1u64, vec![0u8; 10]).hash();

    assert_ne!(h1, h2);

    let h1 = 1u64.hash();
    let h2 = vec![0u8; 10].hash();
    let h3 = hash_two!(h1, h2);
    let h4 = (h1, h2).hash();

    assert_eq!(h3, h4);
}

#[test]
fn hash_with_index_works() {
    let h1 = vec![1u8; 10].hash();
    let h2 = vec![1u8; 10].hash();

    assert_eq!(h1, h2);

    let want = hash_two!(1u64.to_le_bytes(), &h1);
    let got = hash_with_index(1, &h1);
    assert_eq!(want, got);

    let want = hash_two!(2u64.to_le_bytes(), &h2);
    let got = hash_with_index(2, &h2);
    assert_eq!(want, got);
}
