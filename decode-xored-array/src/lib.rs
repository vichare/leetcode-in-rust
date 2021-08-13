//! https://leetcode.com/problems/shuffle-string/

pub struct Solution {}

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut decoded = Vec::with_capacity(encoded.len() + 1);
        decoded.push(first);
        let _ = encoded.iter().fold(
            first,
            |prev: i32, value: &i32| -> i32 {
                decoded.push(prev ^ value);
                prev ^ value
            }
        );
        decoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::decode(vec![1, 2, 3], 1),
            vec![1, 0, 2, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::decode(vec![6, 2, 7, 3], 4),
            vec![4, 2, 0, 7, 4]);
    }
}
