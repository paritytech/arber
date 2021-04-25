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

/// Return true if the node at `pos` is a leaf node.
///
/// This is a convenience wrapper around [`node_height`]
pub(crate) fn is_leaf(pos: u64) -> bool {
    node_height(pos) == 0
}

/// Return the height of the MMR peaks **before** a node at (0-based) index `idx`
/// is added as well as the height node `idx` itself will be added.
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
/// The new node itself will be positioned at height 0.
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

/// For a given node at position `pos` calculate the parent and sibling positions
///  for a path up to the `end_pos` node. This `family_path` will contain the nodes
/// needed in order to generate a membership Merkle proof for the node at `pos`.
///
/// For example, given the tree below, the family path for position '8' would be
/// '9 - 13 - 7'.
///
///```no
///               15
///            /      \
///           /        \
///          /          \
///         /            \
///        7             14
///      /    \        /    \
///     3      6      10     13     
///    / \    /  \   /  \   /  \   
///   1   2  4    5 8    9 11  12
///````
/// The returned family path is encoded as a vector of tuples. Each tuple is of
/// the form `(parent, sibling)`, where `sibling` is the position of the tree node
/// needed in order to calculate the hash for `parent`. Starting with the node
/// at position `pos`.
///
/// For example, given the tree above and starting at node '8', the encoded family
/// path will look like:
/// ```no
/// [(10, 9), (14, 13), (15, 7)]
/// ```
pub(crate) fn family_path(pos: u64, end_pos: u64) -> Vec<(u64, u64)> {
    let mut path = vec![];
    let (peak_map, node_height) = peak_height_map(pos.saturating_sub(1));
    let mut parent_height = 1 << node_height;
    let mut node_pos = pos;
    let mut sibling;

    while node_pos < end_pos {
        if (peak_map & parent_height) != 0 {
            node_pos += 1;
            sibling = node_pos - 2 * parent_height;
        } else {
            node_pos += 2 * parent_height;
            sibling = node_pos - 1;
        };

        if node_pos > end_pos {
            break;
        }

        path.push((node_pos, sibling));
        parent_height <<= 1;
    }

    path
}

#[cfg(test)]
mod tests {
    use super::{family_path, is_leaf, node_height, peak_height_map, peaks};

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
    fn is_leaf_works() {
        assert!(is_leaf(0));
        assert!(is_leaf(1));
        assert!(is_leaf(2));
        assert!(!is_leaf(3));
        assert!(is_leaf(4));
        assert!(is_leaf(5));
        assert!(!is_leaf(6));
        assert!(!is_leaf(7));
        assert!(is_leaf(8));
        assert!(!is_leaf(10));
        assert!(!is_leaf(15));
        assert!(is_leaf(16));
        assert!(!is_leaf(18));
        assert!(is_leaf(19));
        assert!(!is_leaf(28));
        assert!(!is_leaf(29));
        assert!(!is_leaf(30));
        assert!(!is_leaf(31));
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

    #[test]
    fn family_path_works() {
        let path = family_path(1, 3);
        assert_eq!(vec![(3, 2)], path);

        let path = family_path(1, 7);
        assert_eq!(vec![(3, 2), (7, 6)], path);

        let path = family_path(1, 15);
        assert_eq!(vec![(3, 2), (7, 6), (15, 14)], path);

        let path = family_path(8, 15);
        assert_eq!(vec![(10, 9), (14, 13), (15, 7)], path);
    }

    #[test]
    fn family_path_invalid_args() {
        const EMPTY: Vec<(u64, u64)> = vec![];

        let path = family_path(1, 2);
        assert_eq!(EMPTY, path);

        let path = family_path(0, 0);
        assert_eq!(EMPTY, path);

        let path = family_path(12, 2);
        assert_eq!(EMPTY, path)
    }
}
