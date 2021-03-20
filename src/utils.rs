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

#[cfg(test)]
mod tests {
    use super::{node_height, peaks};

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
}
