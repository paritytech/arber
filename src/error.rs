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

//! Merkle-Mountain-Range errors

use core::{
    marker::{Send, Sync},
    result, write,
};

use displaydoc::Display;

use crate::{Hash, String};

#[derive(Display, Debug, PartialEq, Eq, Clone)]
pub enum Error {
    #[displaydoc("expecting leaf node at pos: {0}")]
    ExpectingLeafNode(u64),
    #[displaydoc("invalid hex string: {0}")]
    InvalidHexString(String),
    #[displaydoc("invalid node hash at idx {0}: {1} != {2}")]
    InvalidNodeHash(u64, Hash, Hash),
    #[displaydoc("invalid node height: {0}")]
    InvalidNodeHeight(u64),
    #[displaydoc("invalid root hash: {0} != {1}")]
    InvalidRootHash(Hash, Hash),
    #[displaydoc("missing hash at index: {0}")]
    MissingHashAtIndex(u64),
    #[displaydoc("missing root node")]
    MissingRootNode,
}

unsafe impl Send for Error {}

unsafe impl Sync for Error {}

/// A specialized [`core::result::Result`] type for MMR operations.
///
/// This type is used for any MMR operation which may produce an error.
///
/// While usual Rust style is to import types directly, aliases of [`core::result::Result`] often
/// are not, to make it easier to distinguish between them.
///
pub type Result<T> = result::Result<T, Error>;
