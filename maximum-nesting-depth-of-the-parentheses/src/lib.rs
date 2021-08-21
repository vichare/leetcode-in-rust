//! https://leetcode.com/problems/shuffle-string/

pub struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut depth = 0_i32;
        let mut max_depth = 0_i32;
        for c in s.chars() {
            match c {
                '(' => {
                    depth += 1;
                    if depth > max_depth {
                        max_depth = depth;
                    }
                },
                ')' => {
                    depth -= 1;
                    if depth < 0 {
                        return -1;
                    }
                },
                _ => {},
            }
        }
        max_depth
    }
}
