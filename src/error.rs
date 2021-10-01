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

//! Merkle-Mountain-Range errors

use core::{
    marker::{Send, Sync},
    write,
};

use displaydoc::Display;

use crate::String;

#[derive(Display, Debug, PartialEq, Eq, Clone)]
pub enum Error {
    #[displaydoc("invalid node height: {0}")]
    InvalidNodeHeight(u64),
    #[displaydoc("missing hash at index: {0}")]
    MissingHashAtIndex(u64),
    #[displaydoc("store error: {0}")]
    Store(String),
    #[displaydoc("validation error: {0}")]
    Validate(String),
    #[displaydoc("hex parse error: {0}")]
    ParseHex(String),
    #[displaydoc("merkle proof error: {0}")]
    Proof(String),
    #[displaydoc("invalid MMR error: {0}")]
    Invalid(String),
}

unsafe impl Send for Error {}

unsafe impl Sync for Error {}
