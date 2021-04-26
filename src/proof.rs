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

//! Merkle Proof for a MMR path

use crate::{error::Error, hash_with_index, utils, Hash, Hashable};

#[derive(Clone, Debug, PartialEq)]
pub struct MerkleProof {
    pub mmr_size: u64,
    pub path: Vec<Hash>,
}

impl Default for MerkleProof {
    fn default() -> Self {
        MerkleProof::new()
    }
}

impl MerkleProof {
    pub fn new() -> MerkleProof {
        MerkleProof {
            mmr_size: 0,
            path: Vec::default(),
        }
    }

    /// Verfiy that `elem` is a MMR node at positon `pos` given the root hash `root`.
    pub fn verify(&self, root: Hash, elem: &dyn Hashable, pos: u64) -> Result<bool, Error> {
        let peaks = utils::peaks(self.mmr_size);
        self.clone().do_verify(root, elem, pos, &peaks)
    }

    fn do_verify(
        &mut self,
        root: Hash,
        elem: &dyn Hashable,
        pos: u64,
        _peaks: &[u64],
    ) -> Result<bool, Error> {
        let hash = if pos > self.mmr_size {
            hash_with_index(self.mmr_size, &elem.hash())
        } else {
            hash_with_index(pos - 1, &elem.hash())
        };

        // MMR has only a single node
        if self.path.is_empty() {
            if root == hash {
                return Ok(true);
            } else {
                return Err(Error::Proof(format!("root mismatch {} != {}", hash, root)));
            }
        }

        Ok(false)
    }
}
