// https://leetcode.com/problems/longest-palindromic-substring/

// time: O()
// mem: O()
// given a str, return the first longest palindomic substr
// all chars are [a-Z]|[0-9]
fn longest_palindrome(s: &str) -> &str {
    let mut longest = "";

    // TODO: solve this haha lol

    // even simple
    // abccbd - a
    // abccbd - ab
    // abccbd - abc
    // abccbd - abcc
    // abccbd - abccb
    // abccbd x abccbd
    // abccbd ! bccb

    // odd simple
    // abcbd - a
    // abcbd - ab
    // abcbd - abc
    // abcbd - abcb
    // abcbd x abcbd
    // abcbd ! bcb

    // odd nested
    // bcbdedbcb - b
    // bcbdedbcb - bc
    // bcbdedbcb - bcb ! sub palindrome
    // bcbdedbcb - bcbd
    // bcbdedbcb - bcbde
    // bcbdedbcb - bcbded
    // bcbdedbcb - bcbdedb
    // bcbdedbcb - bcbdedbc
    // bcbdedbcb - bcbdedbcb
    // bcbdedbcb ! bcbdedbcb
    // 010232010 assigning increasing numbers will not help on alternating sub palindromes
    // 010101010

    // use deque instead of a stack, or keep around indecies and
    // just use a str that get's shifted and resized depending on the chars
    // problem: when to clear deque and assume longest candidate

    // n² solution would be checking outwards from every char (n * n / 2)

    for byte in s.as_bytes() {
        let candidate = "";
        if candidate.len() > longest.len() {
            longest = candidate;
        }
    }
    ""
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let string = "";
        let p = longest_palindrome(string);

        let res = "";
        assert_eq!(p, res);
    }

    #[test]
    fn single_char() {
        let string = "a";
        let p = longest_palindrome(string);

        let res = "a";
        assert_eq!(p, res);
    }

    #[test]
    fn simple() {
        let string = "cbbd";
        let p = longest_palindrome(string);

        let res = "bb";
        assert_eq!(p, res);
    }

    #[test]
    fn mutliple_single_chars() {
        let string = "ac";
        let p = longest_palindrome(string);

        let res = "a";
        assert_eq!(p, res);
    }

    #[test]
    fn first_non_simple() {
        let string = "babad";
        let p = longest_palindrome(string);

        let res = "bab";
        assert_eq!(p, res);
    }
}
