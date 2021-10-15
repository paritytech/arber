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

use codec::{Decode, Encode};

use crate::{vec, Error, Hash, Result, Vec};

#[cfg(test)]
#[path = "store_tests.rs"]
mod tests;

pub trait MutableStore<T>
where
    T: Clone + Decode + Encode,
{
    fn append(&mut self, elem: &T, hashes: &[Hash]) -> Result<()>;

    fn hash_at(&self, idx: u64) -> Result<Hash>;
}

pub struct VecStore<T> {
    /// Optional store elements, `None` if only hashes are stored.
    pub data: Option<Vec<T>>,
    /// MMR hashes for both, laves and parents
    pub hashes: Vec<Hash>,
}

impl<T> MutableStore<T> for VecStore<T>
where
    T: Clone + Decode + Encode,
{
    fn append(&mut self, elem: &T, hashes: &[Hash]) -> Result<()> {
        if let Some(data) = &mut self.data {
            data.push(elem.clone());
        }

        self.hashes.extend_from_slice(hashes);

        Ok(())
    }

    fn hash_at(&self, idx: u64) -> Result<Hash> {
        self.hashes
            .get(idx as usize)
            .cloned()
            .ok_or(Error::MissingHashAtIndex(idx))
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
