// time: O(2 * (n / k) + (n - (2n / k)))
// mem: O(n)
// given a str, return the longst substr without a duplicate char
// all chars are alphabetic and `,` & `.`
pub fn convert(s: &str, num_rows: usize) -> String {
    // TODO: can this be done in place?
    let mut string = String::with_capacity(s.len());

    // all chars are within valid ascii
    let bytes = s.as_bytes();

    // offset must not be 0
    let max_offset = match (2 * num_rows) - 2 {
        0 => 1,
        x => x,
    };

    for n in 0..num_rows {
        let offset = (2 * (num_rows - n)) - 2;
        for i in (0..)
            .map(|i| n + (i * max_offset))
            .take_while(|&i| i < s.len())
        {
            string.push(bytes[i] as char);
            let i_o = i + offset;

            // ignore last and first row as they one char per period
            if n != 0 && n != num_rows - 1 && i_o < s.len() {
                string.push(bytes[i_o] as char);
            }
        }
    }

    string
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_row() {
        let string = "A";
        let num_rows = 1;

        let res = convert(string, num_rows);
        assert_eq!(res, "A");
    }

    #[test]
    fn two_rows() {
        let string = "PAYPALISHIRING";
        let num_rows = 2;

        let res = convert(string, num_rows);
        assert_eq!(res, "PYAIHRNAPLSIIG");

        // P Y A I H R N
        // A P L S I I G
    }

    #[test]
    fn three_rows() {
        let string = "PAYPALISHIRING";
        let num_rows = 3;

        let res = convert(string, num_rows);
        assert_eq!(res, "PAHNAPLSIIGYIR");

        // P   A   H   N
        // A P L S I I G
        // Y   I   R
    }

    #[test]
    fn four_rows() {
        let string = "PAYPALISHIRING";
        let num_rows = 4;

        let res = convert(string, num_rows);
        assert_eq!(res, "PINALSIGYAHRPI");

        // P     I    N
        // A   L S  I G
        // Y A   H R
        // P     I
    }
}
