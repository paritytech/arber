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

//! Hash type

use {
    crate::Error,
    std::{
        cmp::min,
        fmt::{self, Write},
    },
};

macro_rules! to_hex {
    ($bytes:expr) => {{
        let mut s = std::string::String::with_capacity(64);

        for b in $bytes {
            std::write!(&mut s, "{:02x}", b)?
        }

        Ok(s)
    }};
}

/// Generic hash type which should be compatible with most hashes used
/// within the blockchain domain.
#[derive(Copy, Clone, PartialEq)]
pub struct Hash([u8; 32]);

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DISP_SIZE: usize = 12;

        let hex = to_hex!(&self.0)?;
        write!(f, "{}", &hex[..DISP_SIZE])
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Hash {
    /// 32 byte hash
    pub const LEN: usize = 32;

    /// Return a hash initialized from `v`.
    ///
    /// At most, up to [`Hash::LEN`] bytes will be copied from `v`. If `v` is
    /// shorter, the hass will be padded with 0's
    pub fn form_vec(v: &[u8]) -> Hash {
        let mut h = [0; Hash::LEN];
        let sz = min(v.len(), Hash::LEN);
        h[..sz].copy_from_slice(&v[..sz]);
        Hash(h)
    }

    pub fn from_hex(hex: &str) -> Result<Hash, Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Hash;

    #[test]
    fn from_vec_works() {
        let v = vec![1, 2, 3];
        let h = format!("{}", Hash::form_vec(&v));
        assert_eq!(h, "010203000000");

        let v = Vec::new();
        let h = format!("{}", Hash::form_vec(&v));
        assert_eq!(h, "000000000000");

        let v = vec![222, 173, 202, 254, 186, 190];
        let h = format!("{}", Hash::form_vec(&v));
        assert_eq!(h, "deadcafebabe");
    }
}
