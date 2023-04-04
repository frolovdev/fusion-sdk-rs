use std::str::FromStr;

use crate::constants::NATIVE_CURRENCY;

pub fn is_native_currency(address: &str) -> bool {
    address.to_lowercase() == NATIVE_CURRENCY
}

pub fn to_sec<T: Into<u64>>(time: T) -> u64 {
    let t = time.into();
    t / 1000
}

// pub fn to_bn<T: AsRef<str>>(val: T) -> BigUint {
//     let val = val.as_ref();
//     if let Ok(n) = val.parse::<u64>() {
//         return n.to_biguint().expect("Failed to convert to biguint");
//     }
//     let val = trim_0x(val);
//     BigUint::from_str(val).expect("Invalid hex string")
// }

pub fn trim_0x(data: &str) -> &str {
    if data.starts_with("0x") {
        &data[2..]
    } else {
        data
    }
}

pub fn add_0x(data: &str) -> String {
    if data.contains("0x") {
        data.to_string()
    } else {
        "0x".to_owned() + data
    }
}

pub trait PadStart {
    fn pad_start(&self, width: usize, fill: char) -> String;
}

impl PadStart for str {
    fn pad_start(&self, width: usize, fill: char) -> String {
        if self.len() >= width {
            self[self.len() - width..self.len()].to_string()
        } else {
            let pad_len = width - self.len();
            let padded: String = std::iter::repeat(fill)
                .take(pad_len)
                .chain(self.chars())
                .collect();
            padded
        }
    }
}

pub trait Substring {
    /// Obtains a string slice containing the characters within the range specified by
    /// `start_index` and `end_index`.
    ///
    /// The range specified is a character range, not a byte range.
    fn substring(&self, start_index: usize, end_index: usize) -> &str;
}

impl Substring for str {
    fn substring(&self, start_index: usize, end_index: usize) -> &str {
        if end_index <= start_index {
            return "";
        }

        let mut indices = self.char_indices();

        let obtain_index = |(index, _char)| index;
        let str_len = self.len();

        unsafe {
            // SAFETY: Since `indices` iterates over the `CharIndices` of `self`, we can guarantee
            // that the indices obtained from it will always be within the bounds of `self` and they
            // will always lie on UTF-8 sequence boundaries.
            self.get_unchecked(
                indices.nth(start_index).map_or(str_len, &obtain_index)
                    ..indices
                        .nth(end_index - start_index - 1)
                        .map_or(str_len, &obtain_index),
            )
        }
    }
}

#[cfg(test)]
mod tests {

    mod pad_start {
        use super::super::PadStart;

        #[test]
        fn test_pad_start_no_padding_needed() {
            let input = "Hello";
            let expected = "Hello".to_string();
            assert_eq!(input.pad_start(5, '0'), expected);
        }

        #[test]
        fn test_pad_start_padding_with_zeros() {
            let input = "42";
            let expected = "00042".to_string();
            assert_eq!(input.pad_start(5, '0'), expected);
        }

        #[test]
        fn test_pad_start_padding_with_spaces() {
            let input = "Hello";
            let expected = "     Hello".to_string();
            assert_eq!(input.pad_start(10, ' '), expected);
        }

        #[test]
        fn test_pad_start_padding_with_custom_character() {
            let input = "Rust";
            let expected = "----Rust".to_string();
            assert_eq!(input.pad_start(8, '-'), expected);
        }
    }

    mod substring {
        use super::super::Substring;
        #[test]
        fn test_substring() {
            assert_eq!("foobar".substring(0, 3), "foo");
        }

        #[test]
        fn test_out_of_bounds() {
            assert_eq!("foobar".substring(0, 10), "foobar");
            assert_eq!("foobar".substring(6, 10), "");
        }

        #[test]
        fn test_start_less_than_end() {
            assert_eq!("foobar".substring(3, 2), "");
        }

        #[test]
        fn test_start_and_end_equal() {
            assert_eq!("foobar".substring(3, 3), "");
        }

        #[test]
        fn test_multiple_byte_characters() {
            assert_eq!("fõøbα®".substring(2, 5), "øbα");
        }
    }
}
