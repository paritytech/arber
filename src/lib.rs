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

//! Merkle-Mountain-Range implementation.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!("std.rs");

#[cfg(not(feature = "std"))]
include!("no_std.rs");

pub use error::{Error, Result};
pub use hash::{hash_with_index, Hash, Hashable};
pub use mmr::MerkleMountainRange;
pub use proof::MerkleProof;
pub use store::{Store, VecStore};

mod error;
mod hash;
mod mmr;
mod proof;
mod store;
mod utils;
