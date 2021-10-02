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

//! Merkle Proof for a MMR path

use codec::{Decode, Encode};

use crate::{error::Error, hash_with_index, utils, Hash, Hashable, Vec};

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
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
        peaks: &[u64],
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
                return Err(Error::InvalidRootHash(hash, root));
            }
        }

        let sibling = self.path.remove(0);
        let (parent_pos, sibling_pos) = utils::family(pos);

        if let Ok(x) = peaks.binary_search(&pos) {
            let parent = if x == peaks.len() - 1 {
                (sibling, hash)
            } else {
                (hash, sibling)
            };
            self.verify(root, &parent, parent_pos)
        } else if parent_pos > self.mmr_size {
            let parent = (sibling, hash);
            self.verify(root, &parent, parent_pos)
        } else {
            let parent = if utils::is_left(sibling_pos) {
                (sibling, hash)
            } else {
                (hash, sibling)
            };
            self.verify(root, &parent, parent_pos)
        }
    }
}
