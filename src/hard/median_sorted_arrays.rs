use std::cmp::Ordering;

// https://leetcode.com/problems/median-of-two-sorted-arrays/

// time: O(1)
// mem: O(1)
#[inline]
fn median_single(nums: &Vec<i32>) -> f64 {
    // on odd med will be rounded down to the correct index
    // on even med will be the higher part
    let med = nums.len() / 2;
    if nums.len() % 2 == 0 {
        // len 4, med 2 & 3
        // [xxxx] 1 & 2
        (nums[med - 1] as f64 + nums[med] as f64) / 2.0
    } else {
        // len 3, med 2
        // [xxx] 1
        nums[med] as f64
    }
}

// time: O(1)
// mem: O(1)
#[inline]
fn ordered(first: &Vec<i32>, second: &Vec<i32>) -> f64 {
    let len = first.len() + second.len();
    let med = len / 2;
    if len % 2 == 0 {
        // total len 6, med 3 & 4
        // [xxxxx][x] f 2 & f 3
        // [x][xxxxx] s 1 & s 2
        // [xxx][xxx] f 2 & s 0

        match med.cmp(&first.len()) {
            // both values are in the second
            Ordering::Greater => {
                (second[med - first.len()] as f64 + first[(med - first.len()) + 1] as f64) / 2.0
            }
            // the values are split between both
            Ordering::Equal => (first[med - 1] as f64 + second[med - first.len()] as f64) / 2.0,
            // both values are in the first
            Ordering::Less => (first[med] as f64 + first[med + 1] as f64) / 2.0,
        }
    } else {
        // total len 5, med 3
        // [xxx][xx] f 3
        // [xx][xxx] s 0

        if med >= first.len() {
            second[med - first.len()] as f64
        } else {
            first[med] as f64
        }
    }
}

// time: O((n + m) / 2)
// mem: O(1)
// given 2 sorted vectors
pub fn find_median_sorted_arrays(nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
    // no elements in one or both vectors
    match (nums1.len(), nums2.len()) {
        (0, 0) => return 0.0,
        (_, 0) => return median_single(nums1),
        (0, _) => return median_single(nums2),
        _ => {}
    }

    let (f1, l1) = (nums1[0], nums1[nums1.len() - 1]);
    let (f2, l2) = (nums2[0], nums2[nums2.len() - 1]);

    // vectors must be directly adjacent
    match (f2.cmp(&l1), f1.cmp(&l2)) {
        // nums1, nums2
        (Ordering::Greater | Ordering::Equal, _) => return ordered(nums1, nums2),
        // nums2, nums1
        (_, Ordering::Greater | Ordering::Equal) => return ordered(nums2, nums1),
        // undordered
        _ => {}
    }

    // TODO: is O(log (n + m)) really possible?
    // fastest I can do is O((n + m) / 2) by incrementally going up the vectors to the median
    let len_merge = nums1.len() + nums2.len();
    let (mut i1, mut i2) = (0, 0);
    let (li1, li2) = (nums1.len() - 1, nums2.len() - 1);
    let mut med = 0;

    while i1 + i2 < len_merge / 2 {
        let (n1, n2) = (nums1[i1], nums2[i2]);

        // take smaller or equal
        med = if Ord::cmp(&n1, &n2) != Ordering::Greater {
            i1 += 1;
            n1
        } else {
            i2 += 1;
            n2
        };
    }

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
        let m = find_median_sorted_arrays(&nums1, &nums2);
        assert_eq!(m, 0.0);
    }

    #[test]
    fn empty_odd() {
        let (nums1, nums2) = (vec![0], vec![]);
        let m = find_median_sorted_arrays(&nums1, &nums2);
        assert_eq!(m, 0.0);
    }

    #[test]
    fn empty_even() {
        let (nums1, nums2) = (vec![1, 2], vec![]);
        let m = find_median_sorted_arrays(&nums1, &nums2);
        assert_eq!(m, 1.5);
    }

    #[test]
    fn ordered_odd() {
        let (nums1, nums2) = (vec![1, 2], vec![3]);
        let m = find_median_sorted_arrays(&nums1, &nums2);
        assert_eq!(m, 2.0);
    }

    #[test]
    fn ordered_even() {
        let (nums1, nums2) = (vec![1, 2], vec![3, 4]);
        let m = find_median_sorted_arrays(&nums1, &nums2);
        assert_eq!(m, 2.5);
    }

    #[test]
    fn perfect_zip_even() {
        let (nums1, nums2) = (vec![1, 3, 5], vec![2, 4, 6]);
        let m = find_median_sorted_arrays(&nums1, &nums2);
        assert_eq!(m, 3.5);
    }

    #[test]
    fn partial_zip_odd() {
        let (nums1, nums2) = (vec![1, 3, 6, 8], vec![2, 7]);
        let m = find_median_sorted_arrays(&nums1, &nums2);
        assert_eq!(m, 4.5);
    }
}
