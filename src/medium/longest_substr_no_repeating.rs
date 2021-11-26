use std::collections::HashSet;

// https://leetcode.com/problems/longest-substring-without-repeating-characters/

// the problem asked for the length, but returning a str encapsulates both it's start and it's length cheaply

// time: O(n)..O(2n)
// mem: O(1)..O(n)
// given a str, return the longst substr without a duplicate char
// all chars are valid ascii
pub fn length_of_longest_substring(s: &str) -> &str {
    let mut chars = HashSet::new();
    let mut partitions = vec![];

    let mut prev = 0;
    for (idx, byte) in s.as_bytes().iter().enumerate() {
        if !chars.insert(byte) {
            // already known char, new partition
            partitions.push(&s[prev..idx]);
            chars.clear();
            chars.insert(byte);
            prev = idx;
        }
    }

    partitions
        .into_iter()
        .max_by(|prev, part| prev.len().cmp(&part.len()))
        .unwrap_or("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let l = length_of_longest_substring("abcabcbb");
        assert_eq!(l, "abc");
    }

    #[test]
    fn single_repeating() {
        let l = length_of_longest_substring("bbbbb");
        assert_eq!(l, "b");
    }

    #[test]
    fn substr_not_subseq() {
        let l = length_of_longest_substring("pwwkew");
        assert_eq!(l, "wke");
    }

    #[test]
    fn empty() {
        let l = length_of_longest_substring("");
        assert_eq!(l, "");
    }
}
