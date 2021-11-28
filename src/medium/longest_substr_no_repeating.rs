use std::collections::HashSet;

// https://leetcode.com/problems/longest-substring-without-repeating-characters/

// the problem asked for the length, but returning a str encapsulates both it's start and it's length cheaply

// time: O(n)..O(2n)
// mem: O(1)..O(n)
// given a str, return the longst substr without a duplicate char
// all chars are valid ascii
fn length_of_longest_substring(s: &str) -> &str {
    let mut chars = HashSet::new();
    let mut parts = vec![];

    let mut prev = 0;
    for (idx, byte) in s.as_bytes().iter().enumerate() {
        if !chars.insert(byte) {
            // already known char, new partition
            parts.push(&s[prev..idx]);
            chars.clear();
            chars.insert(byte);
            prev = idx;
        }
    }

    parts
        .into_iter()
        .max_by_key(|part| part.len())
        .unwrap_or("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let string = "abcabcbb";
        let substr = length_of_longest_substring(string);

        let res = "abc";
        assert_eq!(substr, res);
    }

    #[test]
    fn single_repeating() {
        let string = "bbbbb";
        let substr = length_of_longest_substring(string);

        let res = "b";
        assert_eq!(substr, res);
    }

    #[test]
    fn substr_not_subseq() {
        let string = "pwwkew";
        let substr = length_of_longest_substring(string);

        let res = "wke";
        assert_eq!(substr, res);
    }

    #[test]
    fn empty() {
        let string = "";
        let substr = length_of_longest_substring(string);

        let res = "";
        assert_eq!(substr, res);
    }
}
