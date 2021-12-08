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

//! MMR vector store tests

use arber::{MerkleMountainRange, Result, VecStore};

type E = Vec<u8>;

#[test]
fn append_two_nodes() -> Result<()> {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);

    let n1 = vec![0u8, 10];
    let pos = mmr.append(&n1)?;

    assert_eq!(1, pos);

    let n2 = vec![1u8, 10];
    let pos = mmr.append(&n2)?;

    assert_eq!(3, pos);

    Ok(())
}

#[test]
fn append_multiple_nodes() -> Result<()> {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);
    let mut size = 0;

    for i in 0..=10u8 {
        let n = vec![i, 10];
        size = mmr.append(&n)?;
    }

    assert_eq!(19, size);

    Ok(())
}

#[test]
fn validate() -> Result<()> {
    let mut s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);
    let mut size = 0;

    for i in 0..=2u8 {
        let n = vec![i, 10];
        size = mmr.append(&n)?;
    }

    assert_eq!(4, size);
    assert!(mmr.validate().unwrap());

    s = VecStore::<E>::new();
    mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);
    size = 0;

    for i in 0..=6u8 {
        let n = vec![i, 10];
        size = mmr.append(&n)?;
    }

    assert_eq!(11, size);
    assert!(mmr.validate().unwrap());

    s = VecStore::<E>::new();
    mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);
    size = 0;

    for i in 0..=10u8 {
        let n = vec![i, 10];
        size = mmr.append(&n)?;
    }

    assert_eq!(19, size);
    assert!(mmr.validate().unwrap());

    Ok(())
}

#[test]
fn peaks() -> Result<()> {
    let s = VecStore::<Vec<u32>>::new();
    let mut mmr = MerkleMountainRange::<Vec<u32>, VecStore<Vec<u32>>>::new(0, s);

    for i in 0..=100u32 {
        let n = vec![i, 10];
        mmr.append(&n)?;
    }

    assert_eq!(4, mmr.peaks()?.len());

    for i in 0..=1_000u32 {
        let n = vec![i, 10];
        mmr.append(&n)?;
    }

    assert_eq!(5, mmr.peaks()?.len());

    for i in 0..=10_000u32 {
        let n = vec![i, 10];
        mmr.append(&n)?;
    }

    assert_eq!(10, mmr.peaks()?.len());

    Ok(())
}
