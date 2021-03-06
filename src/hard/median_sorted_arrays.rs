use std::cmp::Ordering;

// https://leetcode.com/problems/median-of-two-sorted-arrays/

// time: O(1)
// mem: O(1)
#[inline]
fn median_single(nums: &Vec<i32>) -> f64 {
    // on odd med will be rounded down to the correct index
    // on even med will be the higher part
    let idx_med = nums.len() / 2;
    if nums.len() % 2 == 0 {
        // even length
        // [xxxx] avg of two
        (nums[idx_med - 1] as f64 + nums[idx_med] as f64) / 2.0
    } else {
        // odd length
        // [xxx] single
        nums[idx_med] as f64
    }
}

// time: O(1)
// mem: O(1)
#[inline]
fn ordered(first: &Vec<i32>, second: &Vec<i32>) -> f64 {
    let len = first.len() + second.len();
    let idx_med = len / 2;
    if len % 2 == 0 {
        // even length
        // [xxxxx][x] both first
        // [x][xxxxx] both second
        // [xxx][xxx] one each

        match idx_med.cmp(&first.len()) {
            Ordering::Greater => {
                (second[idx_med - first.len()] as f64 + second[(idx_med - first.len()) + 1] as f64)
                    / 2.0
            }
            Ordering::Equal => {
                (first[idx_med - 1] as f64 + second[idx_med - first.len()] as f64) / 2.0
            }
            Ordering::Less => (first[idx_med] as f64 + first[idx_med + 1] as f64) / 2.0,
        }
    } else {
        // odd length
        // [xxx][xx] in first
        // [xx][xxx] in second

        if idx_med >= first.len() {
            second[idx_med - first.len()] as f64
        } else {
            first[idx_med] as f64
        }
    }
}

// time: O((n + m) / 2)
// mem: O(1)
// given 2 sorted vectors
fn find_median_sorted_arrays(nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
    // no elements in one or both vec
    match (nums1.len(), nums2.len()) {
        (0, 0) => return 0.0,
        (_, 0) => return median_single(nums1),
        (0, _) => return median_single(nums2),
        _ => {}
    }

    // compare last and first for fast path on totally ordered vecs
    let (f1, l1) = (nums1[0], nums1[nums1.len() - 1]);
    let (f2, l2) = (nums2[0], nums2[nums2.len() - 1]);

    // appending one to the other would result in a sorted vec
    match (f2.cmp(&l1), f1.cmp(&l2)) {
        // nums1, nums2
        (Ordering::Greater | Ordering::Equal, _) => return ordered(nums1, nums2),
        // nums2, nums1
        (_, Ordering::Greater | Ordering::Equal) => return ordered(nums2, nums1),
        // undordered
        _ => {}
    }

    // TODO: is O(log (n + m)) really possible?
    // fastest I can do is O((n + m) / 2) by incrementally going up the vec to the median
    let len_merge = nums1.len() + nums2.len();
    let (mut i1, mut i2) = (0, 0);
    let (li1, li2) = (nums1.len() - 1, nums2.len() - 1);
    let mut med = 0;

    // only walk up to right below the median
    while i1 + i2 < len_merge / 2 {
        let (n1, n2) = (nums1[i1], nums2[i2]);

        // take smaller or equal
        med = if n1 <= n2 {
            i1 += 1;
            n1
        } else {
            i2 += 1;
            n2
        };
    }

    // next one is either median or partial median
    let med_offset = Ord::min(nums1[Ord::min(i1, li1)], nums2[Ord::min(i2, li2)]);
    if len_merge % 2 == 0 {
        (med as f64 + med_offset as f64) / 2.0
    } else {
        med_offset as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn both_empty() {
        let (nums1, nums2) = (vec![], vec![]);
        let median = find_median_sorted_arrays(&nums1, &nums2);

        let res = 0.0;
        assert_eq!(median, res);
    }

    #[test]
    fn empty_odd() {
        let (nums1, nums2) = (vec![0], vec![]);
        let median = find_median_sorted_arrays(&nums1, &nums2);

        let res = 0.0;
        assert_eq!(median, res);
    }

    #[test]
    fn empty_even() {
        let (nums1, nums2) = (vec![1, 2], vec![]);
        let median = find_median_sorted_arrays(&nums1, &nums2);

        let res = 1.5;
        assert_eq!(median, res);
    }

    #[test]
    fn ordered_odd() {
        let (nums1, nums2) = (vec![1, 2], vec![3]);
        let median = find_median_sorted_arrays(&nums1, &nums2);

        let res = 2.0;
        assert_eq!(median, res);
    }

    #[test]
    fn ordered_even() {
        let (nums1, nums2) = (vec![1, 2], vec![3, 4]);
        let median = find_median_sorted_arrays(&nums1, &nums2);

        let res = 2.5;
        assert_eq!(median, res);
    }

    #[test]
    fn perfect_zip_even() {
        let (nums1, nums2) = (vec![1, 3, 5], vec![2, 4, 6]);
        let median = find_median_sorted_arrays(&nums1, &nums2);

        let res = 3.5;
        assert_eq!(median, res);
    }

    #[test]
    fn partial_zip_odd() {
        let (nums1, nums2) = (vec![1, 3, 6, 8], vec![2, 7]);
        let median = find_median_sorted_arrays(&nums1, &nums2);

        let res = 4.5;
        assert_eq!(median, res);
    }
}
