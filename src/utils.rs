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

//! Utiility functions mainly for MMR navigation

use crate::{vec, Vec};

#[cfg(test)]
#[path = "util_tests.rs"]
mod tests;

/// 64-bit all being binary ones: 0b1111111...1
const ALL_ONES: u64 = u64::MAX;

/// Return the positions for all peaks given a MMR with `size` nodes.
///
/// Peaks are listed left to right, starting with the leftmost peak. The leftmost
/// peak is also always the 'highest' peak.
///
/// Note that for an 'unstable' MMR, the result vector will be empty!
///
/// We denote a MMR as unstable if the given number of nodes would lead to two leaf
/// nodes. Two inner nodes at the same height without a parent will also result in
/// an unstable MMR.
///
/// For example, the MMR below with `size = 5` is unstable.
/// ```no
///    3
///   / \
///  1   2   4   5
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

/// Return the height of a node at index `idx`.
///
/// The height is calculated as if the node is part of a fully balanced binary
/// tree and the nodes are visited in postorder traversal.
pub(crate) fn node_height(idx: u64) -> u64 {
    let mut idx = idx;

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

/// Return true if the node at `idx` is a leaf node.
///
/// This is a convenience wrapper around [`node_height`]
pub(crate) fn is_leaf(idx: u64) -> bool {
    node_height(idx) == 0
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

/// Is the node at `pos` the left child node of its parent.
pub(crate) fn is_left(pos: u64) -> bool {
    let (peak_map, node_height) = peak_height_map(pos - 1);
    let peak = 1 << node_height;
    (peak_map & peak) == 0
}

/// For a node at `pos`, calculate the positions of its parent and sibling.
/// Sibling might either be the left or right one, depending on which is
/// missing.
///
/// The family is returned as a tuple of the form `(parent, sibling)`.
pub(crate) fn family(pos: u64) -> (u64, u64) {
    let (peak_map, node_height) = peak_height_map(pos - 1);
    let peak = 1 << node_height;

    if (peak_map & peak) != 0 {
        (pos + 1, pos + 1 - 2 * peak)
    } else {
        (pos + 2 * peak, pos + 2 * peak - 1)
    }
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
