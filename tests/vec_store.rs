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
