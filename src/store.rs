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

//! Merkle-Mountain-Range storage

use crate::{Error, Hash};

pub trait Store<T>
where
    T: Clone,
{
    fn append(&mut self, elem: &T, hashes: &[Hash]) -> Result<(), Error>;

    fn hash_at(&self, idx: u64) -> Result<Hash, Error>;

    fn peak_hash_at(&self, idx: u64) -> Result<Hash, Error>;
}

pub struct VecStore<T> {
    /// Optional store elements, `None` if only hashes are stored.
    pub data: Option<Vec<T>>,
    /// MMR hashes for both, laves and parents
    pub hashes: Vec<Hash>,
}

impl<T> Store<T> for VecStore<T>
where
    T: Clone,
{
    fn append(&mut self, elem: &T, hashes: &[Hash]) -> Result<(), Error> {
        if let Some(data) = &mut self.data {
            data.push(elem.clone());
        }

        self.hashes.extend_from_slice(hashes);

        Ok(())
    }

    fn hash_at(&self, idx: u64) -> Result<Hash, Error> {
        self.hashes
            .get(idx as usize)
            .cloned()
            .ok_or_else(|| Error::Store(format!("missing hash at: {}", idx)))
    }

    fn peak_hash_at(&self, idx: u64) -> Result<Hash, Error> {
        self.hashes
            .get(idx as usize)
            .cloned()
            .ok_or_else(|| Error::Store(format!("missing peak hash at: {}", idx)))
    }
}

impl<T> VecStore<T> {
    pub fn new() -> Self {
        VecStore {
            data: Some(vec![]),
            hashes: vec![],
        }
    }
}

impl<T> Default for VecStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use {
        super::{Error, Store, VecStore},
        crate::Hashable,
    };

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
        let want = Error::Store("missing peak hash at: 3".to_string());

        let store = VecStore::<Vec<u8>>::new();
        let got = store.peak_hash_at(3);

        assert_eq!(Err(want), got);
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
        let want = Error::Store("missing hash at: 3".to_string());

        let store = VecStore::<Vec<u8>>::new();
        let got = store.hash_at(3);

        assert_eq!(Err(want), got);
    }
}
