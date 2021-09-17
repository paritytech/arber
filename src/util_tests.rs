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

//! Utiility functions unit tests

use super::{family, family_path, is_leaf, is_left, node_height, peak_height_map, peaks};

#[test]
fn peaks_works() {
    const UNSTABLE: Vec<u64> = vec![];

    // a MMR with zero nodes is viewed as unstable
    assert_eq!(peaks(0), UNSTABLE);

    assert_eq!(peaks(1), [1]);
    // the canonical unstable case
    assert_eq!(peaks(2), UNSTABLE);
    assert_eq!(peaks(3), [3]);
    assert_eq!(peaks(4), [3, 4]);
    assert_eq!(peaks(5), UNSTABLE);
    assert_eq!(peaks(6), UNSTABLE);
    assert_eq!(peaks(7), [7]);
    assert_eq!(peaks(8), [7, 8]);
    assert_eq!(peaks(9), UNSTABLE);
    assert_eq!(peaks(10), [7, 10]);
    assert_eq!(peaks(11), [7, 10, 11]);
    assert_eq!(peaks(19), [15, 18, 19]);

    let want: Vec<u64> = vec![
        524_287, 786_430, 917_501, 983_036, 1_015_803, 1_032_186, 1_040_377, 1_044_472, 1_046_519,
        1_047_542, 1_048_053, 1_048_308, 1_048_435, 1_048_498, 1_048_529, 1_048_544, 1_048_551,
        1_048_554, 1_048_555,
    ];

    assert_eq!(peaks(1_048_555), want);
}

#[test]
fn node_height_works() {
    assert_eq!(node_height(0), 0);
    assert_eq!(node_height(1), 0);
    assert_eq!(node_height(2), 0);
    assert_eq!(node_height(3), 1);
    assert_eq!(node_height(4), 0);
    assert_eq!(node_height(5), 0);
    assert_eq!(node_height(6), 1);
    assert_eq!(node_height(7), 2);
    assert_eq!(node_height(8), 0);
    assert_eq!(node_height(10), 1);
    assert_eq!(node_height(15), 3);
    assert_eq!(node_height(16), 0);
    assert_eq!(node_height(18), 1);
    assert_eq!(node_height(19), 0);
    assert_eq!(node_height(28), 1);
    assert_eq!(node_height(29), 2);
    assert_eq!(node_height(30), 3);
    assert_eq!(node_height(31), 4);
}

#[test]
fn is_leaf_works() {
    assert!(is_leaf(0));
    assert!(is_leaf(1));
    assert!(is_leaf(2));
    assert!(!is_leaf(3));
    assert!(is_leaf(4));
    assert!(is_leaf(5));
    assert!(!is_leaf(6));
    assert!(!is_leaf(7));
    assert!(is_leaf(8));
    assert!(!is_leaf(10));
    assert!(!is_leaf(15));
    assert!(is_leaf(16));
    assert!(!is_leaf(18));
    assert!(is_leaf(19));
    assert!(!is_leaf(28));
    assert!(!is_leaf(29));
    assert!(!is_leaf(30));
    assert!(!is_leaf(31));
}

#[test]
fn peak_height_map_works() {
    assert_eq!(peak_height_map(0), (0b00, 0));
    assert_eq!(peak_height_map(1), (0b1, 0));
    assert_eq!(peak_height_map(2), (0b1, 1));
    assert_eq!(peak_height_map(3), (0b10, 0));
    assert_eq!(peak_height_map(4), (0b11, 0));
    assert_eq!(peak_height_map(5), (0b11, 1));
    assert_eq!(peak_height_map(6), (0b11, 2));
    assert_eq!(peak_height_map(7), (0b100, 0));
    assert_eq!(peak_height_map(18), (0b1010, 0));

    // test edge cases
    assert_eq!(peak_height_map(u64::MAX), ((u64::MAX >> 1) + 1, 0));
    assert_eq!(peak_height_map(u64::MAX - 1), (u64::MAX >> 1, 63));
}

#[test]
fn is_left_works() {
    assert!(is_left(1));
    assert!(!is_left(2));
    assert!(is_left(3));
    assert!(is_left(4));
    assert!(!is_left(5));
    assert!(!is_left(6));
    assert!(is_left(7));
    assert!(is_left(8));
    assert!(!is_left(9));
    assert!(is_left(10));
    assert!(is_left(11));
    assert!(!is_left(12));
    assert!(!is_left(13));
    assert!(!is_left(14));
    assert!(is_left(15));
}

#[test]
fn family_works() {
    let f = family(1);
    assert_eq!(f, (3, 2));
    let f = family(2);
    assert_eq!(f, (3, 1));

    let f = family(3);
    assert_eq!(f, (7, 6));
    let f = family(6);
    assert_eq!(f, (7, 3));

    let f = family(7);
    assert_eq!(f, (15, 14));
    let f = family(14);
    assert_eq!(f, (15, 7));

    let f = family(11);
    assert_eq!(f, (13, 12));
    let f = family(12);
    assert_eq!(f, (13, 11));
}

#[test]
fn family_path_works() {
    let path = family_path(1, 3);
    assert_eq!(vec![(3, 2)], path);

    let path = family_path(1, 7);
    assert_eq!(vec![(3, 2), (7, 6)], path);

    let path = family_path(1, 15);
    assert_eq!(vec![(3, 2), (7, 6), (15, 14)], path);

    let path = family_path(8, 15);
    assert_eq!(vec![(10, 9), (14, 13), (15, 7)], path);
}

#[test]
fn family_path_invalid_args() {
    const EMPTY: Vec<(u64, u64)> = vec![];

    let path = family_path(1, 2);
    assert_eq!(EMPTY, path);

    let path = family_path(0, 0);
    assert_eq!(EMPTY, path);

    let path = family_path(12, 2);
    assert_eq!(EMPTY, path)
}
