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
    fn example_1() {
        let result = Solution::restore_string(
            "codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]);
        assert_eq!(result, "leetcode".to_string());
    }

    #[test]
    fn example_2() {
        let result = Solution::restore_string(
            "abc".to_string(), vec![0, 1, 2]);
        assert_eq!(result, "abc".to_string());
    }

    #[test]
    fn example_3() {
        let result = Solution::restore_string(
            "aiohn".to_string(), vec![3, 1, 4, 2, 0]);
        assert_eq!(result, "nihao".to_string());
    }

    #[test]
    fn example_4() {
        let result = Solution::restore_string(
            "aaiougrt".to_string(),  vec![4, 0, 2, 6, 7, 3, 1, 5]);
        assert_eq!(result, "arigatou".to_string());
    }

    #[test]
    fn example_5() {
        let result = Solution::restore_string(
            "art".to_string(), vec![1, 0, 2]);
        assert_eq!(result, "rat".to_string());
    }
}
