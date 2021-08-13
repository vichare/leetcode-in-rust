//! https://leetcode.com/problems/shuffle-string/

pub struct Solution {}

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut to_bytes = vec![0_u8; indices.len()];
        for (i, b) in s.as_bytes().iter().enumerate() {
            to_bytes[indices[i] as usize] = *b;
        }
        String::from_utf8(to_bytes).expect("Unable to decode bytes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let result = Solution::restore_string(
            "codeleet".to_string(), vec![4,5,6,7,0,2,1,3]);
        assert_eq!(result, "leetcode".to_string());
    }
}
