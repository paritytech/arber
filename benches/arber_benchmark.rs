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

use {
    arber::{MerkleMountainRange, VecStore},
    criterion::{criterion_group, criterion_main, Criterion},
};

type E = u32;

macro_rules! mmr {
    () => {{
        let s = VecStore::<E>::new();
        MerkleMountainRange::<E, VecStore<E>>::new(s)
    }};
}

fn bench(c: &mut Criterion) {
    c.bench_function("MMR append", |b| {
        b.iter(|| {
            let mut mmr = mmr!();

            for n in 1..=100 {
                mmr.append(&n).unwrap();
            }
        });
    });
}

criterion_group!(benches, bench);

criterion_main!(benches);
