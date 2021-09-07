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

//! arber benchmark

use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};

use arber::{MerkleMountainRange, MerkleProof, VecStore};

type E = u32;

fn make_mmr(num_leafs: u8) -> MerkleMountainRange<E, VecStore<E>> {
    let s = VecStore::<E>::new();
    let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);

    (0..=num_leafs.saturating_sub(1)).for_each(|i| {
        let _ = mmr.append(&(i as u32)).unwrap();
    });

    mmr
}

fn bench(c: &mut Criterion) {
    c.bench_function("MMR append", |b| {
        b.iter(|| {
            let s = VecStore::<E>::new();
            let mut mmr = MerkleMountainRange::<E, VecStore<E>>::new(s);

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
