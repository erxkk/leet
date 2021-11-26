#![feature(
    iter_map_while,
    map_try_insert,
    option_result_unwrap_unchecked,
    box_into_inner
)]
#![allow(dead_code)]

mod easy;
mod hard;
mod medium;

// Leetcode-problems Playground
// https://leetcode.com/problemset/all

// Constraints I set for myself:
// * solve in rust even if unsuitable for the task (eg: building a linked list head first, `medium::add_two_numbers`)
// * optimize to the best of my ability
// * change auto-generated rust code if unidiomatic (eg: use ref isntead of taking ownership, etc.)
// * extend, reduce and/or modify test cases where it makes sense (eg: `hard::median_sorted_arrays`)
// * annotate algorithms with space & time complexity (in a practical sense, _keep_ insignificant terms and factors)
// * add minimal documentation
// * avoid unsafe
