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

use merkle_mountain_range::{Error, MerkleMountainRange, VecStore};

type E = Vec<u8>;

#[test]
fn non_existing_node() {
    let mut s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(&mut s);
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
fn minimal_mmr() {
    let mut s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(&mut s);
    let mut size = 0;

    (0..=1u8).for_each(|i| {
        let n = vec![i];
        size = mmr.append(&n).unwrap();
    });

    let got = mmr.proof(2).unwrap().path[0];
    let got = format!("{}", got);

    assert_eq!("c3264807b084", got);
}
