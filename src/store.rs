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
}

pub(crate) struct VecStore<T> {
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
}

impl<T> VecStore<T> {
    pub fn new() -> Self {
        VecStore {
            data: Some(vec![]),
            hashes: vec![],
        }
    }
}

#[cfg(test)]
mod tests {

    use {
        super::{Store, VecStore},
        crate::Hashable,
    };

    #[test]
    fn append_works() {
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
}
