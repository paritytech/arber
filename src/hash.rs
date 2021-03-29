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
    /// At most, up to [`Hash::LEN`] bytes will be copied from `v`. If `v` has less
    /// than [`Hash::LEN`] bytes, the hash will be padded with 0's from left to right.
    pub fn from_vec(v: &[u8]) -> Hash {
        let mut h = [0; Hash::LEN];
        let sz = min(v.len(), Hash::LEN);
        h[..sz].copy_from_slice(&v[..sz]);
        Hash(h)
    }

    /// Retrun a hash initialized from string `hex`.
    ///
    /// An error is returned, if `hex` is not a well-formed hex string like `"0xcafe"`.
    pub fn from_hex(hex: &str) -> Result<Hash, Error> {
        match parse_hex(hex) {
            Ok(v) => Ok(Hash::from_vec(&v)),
            Err(s) => Err(Error::ParseHex(s)),
        }
    }
}

fn parse_hex(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.trim().trim_start_matches("0x");

    if hex.len() % 2 != 0 {
        Err(hex.to_string())
    } else {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| hex.to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Hash};

    #[test]
    fn from_vec_works() {
        let v = vec![1, 2, 3];
        let h = format!("{}", Hash::from_vec(&v));
        assert_eq!(h, "010203000000");

        let v = Vec::new();
        let h = format!("{}", Hash::from_vec(&v));
        assert_eq!(h, "000000000000");

        let v = vec![222, 173, 202, 254, 186, 190];
        let h = format!("{}", Hash::from_vec(&v));
        assert_eq!(h, "deadcafebabe");
    }

    #[test]
    fn from_hex_works() {
        let want = Hash::from_vec(&vec![]);
        let got = Hash::from_hex("0x00").unwrap();
        assert_eq!(want, got);

        let want = Hash::from_vec(&vec![202, 254]);
        let got = Hash::from_hex("0xcafe").unwrap();
        assert_eq!(want, got);

        let want = Hash::from_vec(&vec![222, 173, 202, 254, 186, 190]);
        let got = Hash::from_hex("0xdeadcafebabe").unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn from_hex_error() {
        let want = Error::ParseHex("000".to_string());
        let got = Hash::from_hex("0x000").err().unwrap();
        assert_eq!(want, got);

        let want = Error::ParseHex("thisisbad".to_string());
        let got = Hash::from_hex("0xthisisbad").err().unwrap();
        assert_eq!(want, got);
    }
}
