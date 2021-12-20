#![allow(dead_code)]

/// Transform a slice of bools into a number
///
/// Most significant bit (MSB) comes first
pub fn bits_to_bytes(bits: &[bool]) -> u64 {
    bits.iter().rev().enumerate().fold(0, |sum, (idx, bit)| {
        if *bit {
            sum + 2u64.pow(idx as u32)
        } else {
            sum
        }
    })
}

pub fn bits_as_u8s_to_bytes(bits: &[u8]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (idx, bit)| match *bit {
            0 => sum,
            1 => sum + 2usize.pow(idx as u32),
            _ => panic!("invalid bit"),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_to_bytes_test() {
        let input: Vec<bool> = [1, 0, 1, 1, 0].iter().map(|&i| i >= 1).collect();
        assert_eq!(bits_to_bytes(&input), 22);
    }
}
