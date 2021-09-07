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

use core::write;

use crate::String;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    Store(String),
    Validate(String),
    ParseHex(String),
    Proof(String),
    Invalid(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::Store(msg) => write!(f, "store error: `{}`", msg)?,
            Error::Validate(msg) => write!(f, "validation error: `{}`", msg)?,
            Error::ParseHex(msg) => write!(f, "failed to parse string as hex: `{}`", msg)?,
            Error::Proof(msg) => write!(f, "merkle proof error: `{}`", msg)?,
            Error::Invalid(msg) => write!(f, "invald MMR: `{}`", msg)?,
        }

        Ok(())
    }
}
