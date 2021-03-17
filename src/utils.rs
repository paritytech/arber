//! Utiility functions mainly for MMR navigation

#![allow(dead_code)]

/// Return the height of a node at `idx`.
pub fn height(mut idx: u64) -> u32 {
    idx += 1;

    let all_ones = |n: u64| n != 0 && n.count_zeros() == n.leading_zeros();

    let jmp_left = |i: u64| {
        let len = 64 - i.leading_zeros();
        let msb = 1 << (len - 1);
        i - (msb - 1)
    };

    while !all_ones(idx) {
        idx = jmp_left(idx)
    }

    64 - idx.leading_zeros() - 1
}

/// Return peak node indices for a MMR with `size` nodes.
pub fn peaks(size: u64) -> Vec<u64> {
    let mut peaks = Vec::new();
    let (mut height, mut idx) = summit(size);

    peaks.push(idx);

    while height > 0 {
        let peak = match next_peak(height, idx, size) {
            Some(peak) => peak,
            None => break, // no more peaks
        };
        height = peak.0;
        idx = peak.1;
        peaks.push(idx);
    }

    peaks
}

/// Return index of the peak at `height`.
///
/// This will always be the largest index that is inside the tree. In other words, the
/// index of the peak will always be less than the total size of the tree.
fn peak_idx(height: u32) -> u64 {
    (1 << (height + 1)) - 2
}

/// Return the index of the next node with and index larger than `idx` that is at the
/// same `height`, i.e. the right sibling.
fn right_sibling(idx: u64, height: u32) -> u64 {
    let d = (2 << height) - 1;
    idx + d
}

/// Return the index of the left child node at `height` of a parent node at `idx`.
///
/// Note that `height` is the height of the child, not the parent itself!
fn left_child(idx: u64, height: u32) -> u64 {
    let d = 2 << height;
    idx - d
}

/// Return the height and index of the highest peak for an MMR with `size` nodes.
///
/// The highest peak will always be the first, leftmost peak. This is because nodes
/// are added from the left to the rright.
fn summit(size: u64) -> (u32, u64) {
    let mut height = 1;
    let mut prev = 0;
    let mut idx = peak_idx(height);

    while idx < size {
        height += 1;
        prev = idx;
        idx = peak_idx(height);
    }

    (height - 1, prev)
}

/// Return the height and index of the next peak, given the `height` and `idx` of the
/// current peak for an MMR with `size` nodes. The next peak is always at the right side
/// of the current peak.
fn next_peak(mut height: u32, mut idx: u64, size: u64) -> Option<(u32, u64)> {
    idx = right_sibling(idx, height);

    while idx > size - 1 {
        if height == 0 {
            return None;
        }
        height -= 1;
        idx = left_child(idx, height);
    }

    Some((height, idx))
}

#[cfg(test)]
mod tests {
    use super::{height, left_child, next_peak, peak_idx, peaks, right_sibling, summit};

    #[test]
    fn height_works() {
        let got = height(0);
        assert_eq!(0, got);

        let got = height(2);
        assert_eq!(1, got);

        let got = height(6);
        assert_eq!(2, got);

        let got = height(14);
        assert_eq!(3, got);

        let got = height(13);
        assert_eq!(2, got);

        let got = height(17);
        assert_eq!(1, got);

        let got = height(18);
        assert_eq!(0, got);
    }

    #[test]
    fn peaks_works() {
        let got = peaks(19);
        assert_eq!(vec![14, 17, 18], got);

        let got = peaks(11);
        assert_eq!(vec![6, 9, 10], got);
    }

    #[test]
    fn peak_idx_works() {
        let got = peak_idx(1);
        assert_eq!(2, got);

        let got = peak_idx(2);
        assert_eq!(6, got);

        let got = peak_idx(3);
        assert_eq!(14, got);
    }

    #[test]
    fn right_sibling_works() {
        let got = right_sibling(0, 0);
        assert_eq!(1, got);

        let got = right_sibling(10, 0);
        assert_eq!(11, got);

        let got = right_sibling(2, 1);
        assert_eq!(5, got);

        let got = right_sibling(9, 1);
        assert_eq!(12, got);

        let got = right_sibling(6, 2);
        assert_eq!(13, got);
    }

    #[test]
    fn left_child_works() {
        let got = left_child(2, 0);
        assert_eq!(0, got);

        let got = left_child(5, 0);
        assert_eq!(3, got);

        let got = left_child(6, 1);
        assert_eq!(2, got);

        let got = left_child(13, 1);
        assert_eq!(9, got);

        let got = left_child(14, 2);
        assert_eq!(6, got);
    }

    #[test]
    fn summit_works() {
        let got = summit(3);
        assert_eq!((1, 2), got);

        let got = summit(7);
        assert_eq!((2, 6), got);

        let got = summit(11);
        assert_eq!((2, 6), got);

        let got = summit(15);
        assert_eq!((3, 14), got);

        let got = summit(18);
        assert_eq!((3, 14), got);
    }

    #[test]
    fn next_peak_works() {
        let got = next_peak(3, 14, 19);
        assert!(matches!(got, Some((1, 17))));

        let got = next_peak(2, 7, 11);
        assert!(matches!(got, Some((1, 10))));
    }
}
