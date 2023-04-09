use std::ops::Add;

use ethers::types::U256;

use crate::constants::{NATIVE_CURRENCY, ZX};

pub fn is_native_currency(address: &str) -> bool {
    address.to_lowercase() == NATIVE_CURRENCY
}

pub fn to_sec<T: Into<u64>>(time: T) -> u64 {
    let t = time.into();
    t / 1000
}

pub fn trim_0x(data: &str) -> &str {
    if data.starts_with(ZX) {
        &data[2..]
    } else {
        data
    }
}

pub fn add_0x(data: &str) -> String {
    if data.contains(ZX) {
        data.to_string()
    } else {
        ZX.to_owned() + data
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

pub fn cumsum<T>(x: &[T]) -> Vec<T>
where
    T: Clone,
    for<'r> &'r T: Add<&'r T, Output = T>,
{
    let mut y = Vec::with_capacity(x.len());

    if !x.is_empty() {
        y.push(x[0].clone());

        for i in 1..x.len() {
            y.push(&y[i - 1] + &x[i]);
        }
    }

    y
}

pub trait Maskn {
    // Return only lowers bits of number (in-place)
    fn maskn(&self, bits: usize) -> Self;
}

impl Maskn for U256 {
    fn maskn(&self, bits: usize) -> Self {
        if self.bits() <= bits {
            return self.to_owned();
        }

        let num = self.to_owned();
        let mask = U256::from((U256::from(1) << U256::from(bits)) - U256::from(1)); // create a mask of the lower `bits` bits
        num & mask
    }
}

#[cfg(test)]
mod tests {
    mod pad_start {
        use super::super::PadStart;
        use pretty_assertions::assert_eq;

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
        use pretty_assertions::assert_eq;
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

    mod cumsum {
        use super::super::cumsum;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_cumsum() {
            let x = vec![1, 2, 3, 4, 5];
            let y = cumsum(&x);
            assert_eq!(y, vec![1, 3, 6, 10, 15]);

            let x1 = vec![6, 10, 3, 2];

            let y1 = cumsum(&x1);

            assert_eq!(y1, vec![6, 16, 19, 21]);
        }

        #[test]
        fn test_cumsum_empty() {
            let x: Vec<i32> = vec![];
            let y: Vec<i32> = cumsum(&x);

            assert_eq!(y, x);
        }
    }

    mod maskn {
        use ethers::types::U256;
        use pretty_assertions::assert_eq;

        use super::super::Maskn;

        #[test]
        fn should_mask_bits_in_place() {
            assert_eq!(U256::from(0).maskn(1), U256::from(0));
            assert_eq!(U256::from(3).maskn(1), U256::from(1));
            assert_eq!(U256::from(4886718345 as u64).maskn(4), U256::from(9));
            assert_eq!(U256::from(4886718345 as u64).maskn(16), U256::from(26505));
            assert_eq!(
                U256::from(4886718345 as u64).maskn(28),
                U256::from(54880137)
            );
        }

        #[test]
        fn should_not_mask_when_number_is_bigger_than_length() {
            assert_eq!(U256::from(0xe3).maskn(56), U256::from(0xe3));
            assert_eq!(U256::from(0xe3).maskn(26), U256::from(0xe3));
        }
    }
}
