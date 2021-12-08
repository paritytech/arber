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

//! arber benchmark

use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};

use arber::{MerkleMountainRange, MerkleProof, VecStore};

type E = u32;

fn make_mmr(num_leafs: u8) -> MerkleMountainRange<E, VecStore<E>> {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);

    (0..=num_leafs.saturating_sub(1)).for_each(|i| {
        let _ = mmr.append(&(i as u32)).unwrap();
    });

    mmr
}

fn bench(c: &mut Criterion) {
    c.bench_function("MMR append", |b| {
        b.iter(|| {
            let s = VecStore::<E>::new();
            let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(0, s);

            for n in 1..=100 {
                mmr.append(&n).unwrap();
            }
        });
    });

    c.bench_function("MMR proof", |b| {
        let mmr = make_mmr(11);
        let leafs = vec![1u64, 2, 4, 5, 8, 9, 11, 12, 16, 17, 19];
        let mut rng = thread_rng();

        b.iter(|| {
            let idx = rng.gen_range(0..=(leafs.len() - 1));
            let _ = mmr.proof(leafs[idx]).unwrap();
        });
    });

    c.bench_function("MMR verfiy", |b| {
        let mmr = make_mmr(11);
        let leafs = vec![1u64, 2, 4, 5, 8, 9, 11, 12, 16, 17, 19];
        let mut proofs = Vec::<MerkleProof>::new();

        leafs.iter().for_each(|l| {
            proofs.push(mmr.proof(*l).unwrap());
        });

        let root = mmr.root().unwrap();
        let mut rng = thread_rng();

        b.iter(|| {
            let idx = rng.gen_range(0..=(proofs.len() - 1));
            let _ = proofs[idx].verify(root, &(idx as u32), leafs[idx]).unwrap();
        });
    });
}

criterion_group!(benches, bench);

criterion_main!(benches);
