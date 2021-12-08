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

//! Merkle-Mountain-Range storage

use codec::{Decode, Encode};

use crate::{vec, Error, Hash, Result, Vec};

#[cfg(test)]
#[path = "store_tests.rs"]
mod tests;

pub trait Store<T>
where
    T: Clone + Decode + Encode,
{
    fn hash_at(&self, index: u64) -> Result<Hash>;

    fn append(&mut self, elem: &T, hashes: &[Hash]) -> Result<()>;
}

pub struct VecStore<T> {
    /// Optional store elements, `None` if only hashes are stored.
    pub data: Option<Vec<T>>,
    /// MMR hashes for both, laves and parents
    pub hashes: Vec<Hash>,
}

impl<T> Store<T> for VecStore<T>
where
    T: Clone + Decode + Encode,
{
    fn hash_at(&self, index: u64) -> Result<Hash> {
        self.hashes
            .get(index as usize)
            .cloned()
            .ok_or(Error::MissingHashAtIndex(index))
    }

    fn append(&mut self, elem: &T, hashes: &[Hash]) -> Result<()> {
        if let Some(data) = &mut self.data {
            data.push(elem.clone());
        }

        self.hashes.extend_from_slice(hashes);

        Ok(())
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
