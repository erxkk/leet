use std::collections::HashMap;

// https://leetcode.com/problems/two-sum/

// the problem asked for an array of indecies but there are always only 2 if exactly
// one solution exists

// time: O(2n)
// mem: O(n)
// given a vector integers, find the indicies of 2 integers that add up to target
// only one solution exits
pub fn two_sum(nums: &Vec<i32>, target: i32) -> (usize, usize) {
    // we don't care about more than 2 appearances per number
    let mut lookup = HashMap::<i32, (usize, usize)>::with_capacity(nums.len());

    // prepare hashmap for quick lookup
    for (idx, elem) in nums.iter().copied().enumerate() {
        if let Some(indecies) = lookup.get_mut(&elem) {
            // update only if second appearance is found, all following appearances
            // are redundant (if onyl one solutioon exists these numbers exist either
            // exactly 2 times or are not the solution)
            if indecies.0 == indecies.1 {
                indecies.1 = idx;
            }
        } else {
            lookup.insert(elem, (idx, idx));
        }
    }

    // find match
    for (idx, current, other) in nums
        .iter()
        .copied()
        .enumerate()
        .map(|(idx, elem)| (idx, elem, target - elem))
    {
        // look if corresponding other exists
        if let Some(indecies) = lookup.get(&other) {
            // different elements xor has mutliple appearance
            // cannot both be true, but both can be false
            if (other != current) ^ (indecies.0 != indecies.1) {
                return (idx, indecies.1);
            }
        }
    }

    // a solutions always exists
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_no_duplicate() {
        let vec = vec![2, 7, 11, 15];
        let target = 9;
        let idx = two_sum(&vec, target);
        assert_eq!(idx, (0, 1));
    }

    #[test]
    fn not_adjacent_no_duplicate() {
        let vec = vec![2, 3, 4];
        let target = 6;
        let indecies = two_sum(&vec, target);
        assert_eq!(indecies, (0, 2));
    }

    #[test]
    fn adjacent_duplicate() {
        let vec = vec![3, 3, 4];
        let target = 6;
        let indecies = two_sum(&vec, target);
        assert_eq!(indecies, (0, 1));
    }

    #[test]
    fn not_adjacent_duplicate() {
        let vec = vec![1, 3, 4, 3, 3];
        let target = 6;
        let indecies = two_sum(&vec, target);
        assert_eq!(indecies, (1, 3));
    }
}
