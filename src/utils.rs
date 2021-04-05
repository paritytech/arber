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

//! Utiility functions mainly for MMR navigation

#![allow(dead_code, unused_macros)]

/// 64-bit all being binary ones: 0b1111111...1
const ALL_ONES: u64 = u64::MAX;

/// Return the indices for all peaks given a MMR with `size` nodes.
///
/// Peaks are listed left to right, starting with the leftmost peak. The leftmost
/// peak is also always the 'highest' peak.
///
/// Note that for an 'unstable' MMR, the result vector will be empty! We denote
/// a MMR as unstable if the given number of nodes would lead to two leaf nodes.
/// For example, the MMR below with `size = 5` is unstable.
/// ```no
///    2
///   / \
///  0   1   3   4
/// ```
pub(crate) fn peaks(size: u64) -> Vec<u64> {
    if size == 0 {
        return vec![];
    }

    let mut peak_idx = ALL_ONES >> size.leading_zeros();
    let mut nodes_left = size;
    let mut prev_peak_idx = 0;
    let mut peaks = vec![];

    while peak_idx != 0 {
        if nodes_left >= peak_idx {
            peaks.push(prev_peak_idx + peak_idx);
            prev_peak_idx += peak_idx;
            nodes_left -= peak_idx;
        }
        peak_idx >>= 1;
    }

    // if, at this point, we have a node left, the MMR is unstable.
    if nodes_left > 0 {
        return vec![];
    }

    peaks
}

/// Return the height of a node at postion `pos`.
///
/// The height is calculated as if the node is part of a fully balanced binary
/// tree and the nodes are visited in postorder traversal.
pub(crate) fn node_height(pos: u64) -> u64 {
    let mut idx = pos.saturating_sub(1);

    if idx == 0 {
        return 0;
    }

    let mut peak_idx = ALL_ONES >> idx.leading_zeros();

    while peak_idx != 0 {
        if idx >= peak_idx {
            idx -= peak_idx;
        }
        peak_idx >>= 1;
    }

    idx
}

/// Return the height of the MMR peaks **before** a node at (0-based) index `idx`
/// is added as well as the height node `pos` itself will be added.
///
/// This information is returned as a tuple of the form `(peak_map, node_height)`.
/// The peak heights are encoded as a bitmap.
///
/// For example `peak_height_map(4)` will return `(0b11, 0)`, as the MMR at this
///  point looked like:
/// ```no
///    2
///   / \
///  0   1   3
/// ```
/// The return value `(0b11, 0)` indicates, that there are peaks at heights 0 and 1.
/// The node itself will be positioned at height 0.
pub(crate) fn peak_height_map(mut idx: u64) -> (u64, u64) {
    if idx == 0 {
        return (0, 0);
    }

    let mut peak_idx = ALL_ONES >> idx.leading_zeros();
    let mut peak_map = 0;

    while peak_idx != 0 {
        peak_map <<= 1;
        if idx >= peak_idx {
            idx -= peak_idx;
            peak_map |= 1;
        }
        peak_idx >>= 1;
    }

    (peak_map, idx)
}

#[cfg(test)]
mod tests {
    use super::{node_height, peak_height_map, peaks};

    #[test]
    fn peaks_works() {
        const UNSTABLE: Vec<u64> = vec![];

        // a MMR with zero nodes is viewed as unstable
        assert_eq!(peaks(0), UNSTABLE);

        assert_eq!(peaks(1), [1]);
        // the canonical unstable case
        assert_eq!(peaks(2), UNSTABLE);
        assert_eq!(peaks(3), [3]);
        assert_eq!(peaks(4), [3, 4]);
        assert_eq!(peaks(5), UNSTABLE);
        assert_eq!(peaks(6), UNSTABLE);
        assert_eq!(peaks(7), [7]);
        assert_eq!(peaks(8), [7, 8]);
        assert_eq!(peaks(9), UNSTABLE);
        assert_eq!(peaks(10), [7, 10]);
        assert_eq!(peaks(11), [7, 10, 11]);
        assert_eq!(peaks(19), [15, 18, 19]);

        let want: Vec<u64> = vec![
            524_287, 786_430, 917_501, 983_036, 1_015_803, 1_032_186, 1_040_377, 1_044_472,
            1_046_519, 1_047_542, 1_048_053, 1_048_308, 1_048_435, 1_048_498, 1_048_529, 1_048_544,
            1_048_551, 1_048_554, 1_048_555,
        ];

        assert_eq!(peaks(1_048_555), want);
    }

    #[test]
    fn node_height_works() {
        assert_eq!(node_height(0), 0);
        assert_eq!(node_height(1), 0);
        assert_eq!(node_height(2), 0);
        assert_eq!(node_height(3), 1);
        assert_eq!(node_height(4), 0);
        assert_eq!(node_height(5), 0);
        assert_eq!(node_height(6), 1);
        assert_eq!(node_height(7), 2);
        assert_eq!(node_height(8), 0);
        assert_eq!(node_height(10), 1);
        assert_eq!(node_height(15), 3);
        assert_eq!(node_height(16), 0);
        assert_eq!(node_height(18), 1);
        assert_eq!(node_height(19), 0);
        assert_eq!(node_height(28), 1);
        assert_eq!(node_height(29), 2);
        assert_eq!(node_height(30), 3);
        assert_eq!(node_height(31), 4);
    }

    #[test]
    fn peak_height_map_works() {
        assert_eq!(peak_height_map(0), (0b00, 0));
        assert_eq!(peak_height_map(1), (0b1, 0));
        assert_eq!(peak_height_map(2), (0b1, 1));
        assert_eq!(peak_height_map(3), (0b10, 0));
        assert_eq!(peak_height_map(4), (0b11, 0));
        assert_eq!(peak_height_map(5), (0b11, 1));
        assert_eq!(peak_height_map(6), (0b11, 2));
        assert_eq!(peak_height_map(7), (0b100, 0));
        assert_eq!(peak_height_map(18), (0b1010, 0));

        // test edge cases
        assert_eq!(peak_height_map(u64::MAX), ((u64::MAX >> 1) + 1, 0));
        assert_eq!(peak_height_map(u64::MAX - 1), (u64::MAX >> 1, 63));
    }
}
