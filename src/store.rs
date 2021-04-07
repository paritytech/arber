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
