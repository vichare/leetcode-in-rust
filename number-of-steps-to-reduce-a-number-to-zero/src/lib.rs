//! https://leetcode.com/problems/shuffle-string/

pub struct Solution {}

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        if num == 0 {
            return 0;
        } else if num == 1 {
            return 1;
        }
        let mut count = 1;
        while num > 1 {
            if num % 2 == 0 {
                count += 1;
            } else {
                count += 2;
            }
            num /= 2;
        }
        count
    }
}
