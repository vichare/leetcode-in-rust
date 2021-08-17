//! https://leetcode.com/problems/shuffle-string/

pub struct Solution {}

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut balance = 0_i32;
        let mut result = 0_i32;
        for c in s.chars() {
            match c {
                'R' => balance += 1,
                'L' => balance -= 1,
                _ => panic!(),
            }
            if balance == 0_i32 {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::balanced_string_split("RLLLLRRRLR".to_string()), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".to_string()), 2);
    }
}
